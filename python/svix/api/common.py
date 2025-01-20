import asyncio
import random
import time
import typing as t
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from http import HTTPStatus

import httpx

from ..internal.openapi_client.client import AuthenticatedClient
from ..internal.openapi_client.errors import UnexpectedStatus
from ..internal.openapi_client.models import HttpError, HTTPValidationError


def ensure_tz(x: t.Optional[datetime]) -> t.Optional[datetime]:
    if x is None:
        return None

    if x.tzinfo is None:
        return x.replace(tzinfo=timezone.utc)
    return x


def sanitize_field(v: t.Any) -> t.Any:
    if isinstance(v, datetime):
        return ensure_tz(v)

    return v


@dataclass
class BaseOptions:
    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: sanitize_field(v) for k, v in asdict(self).items() if v is not None}


@dataclass
class ListOptions(BaseOptions):
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None


@dataclass
class PostOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client

    def _get_httpx_kwargs(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]],
        query_params: t.Optional[t.Dict[str, str]],
        header_params: t.Optional[t.Dict[str, str]],
        json_body: t.Optional[t.Dict[str, t.Any]],
    ) -> t.Dict[str, t.Any]:
        if path_params is None:
            url = f"{self._client.base_url}{path}"
        else:
            formatted_path = path.format(**path_params)
            url = url = f"{self._client.base_url}{formatted_path}"

        headers: t.Dict[str, str] = {
            **self._client.get_headers(),
            "svix-req-id": f"{random.getrandbits(32)}",
        }
        if header_params is not None:
            headers.update(header_params)

        httpx_kwargs = {
            "method": method.upper(),
            "url": url,
            "headers": headers,
            "cookies": self._client.get_cookies(),
            "timeout": self._client.get_timeout(),
            "follow_redirects": self._client.follow_redirects,
            "verify": self._client.verify_ssl,
        }

        if query_params is not None:
            httpx_kwargs["params"] = query_params

        if json_body is not None:
            httpx_kwargs["json"] = json_body
        return httpx_kwargs

    async def _execute_request_asyncio(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]] = None,
        query_params: t.Optional[t.Dict[str, str]] = None,
        header_params: t.Optional[t.Dict[str, str]] = None,
        json_body: t.Optional[t.Dict[str, t.Any]] = None,
    ) -> httpx.Response:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            path_params=path_params,
            query_params=query_params,
            header_params=header_params,
            json_body=json_body,
        )

        response = httpx.request(**httpx_kwargs)

        async with httpx.AsyncClient(verify=self._client.verify_ssl) as _client:
            response = await _client.request(**httpx_kwargs)

            for retry_count, sleep_time in enumerate(self._client.retry_schedule):
                if response.status_code < 500:
                    break

                await asyncio.sleep(sleep_time)
                httpx_kwargs["headers"]["svix-retry-count"] = str(retry_count)
                response = await _client.request(**httpx_kwargs)

        return _filter_response_for_errors_response(response)

    def _execute_request_sync(
        self,
        method: str,
        path: str,
        path_params: t.Optional[t.Dict[str, str]] = None,
        query_params: t.Optional[t.Dict[str, str]] = None,
        header_params: t.Optional[t.Dict[str, str]] = None,
        json_body: t.Optional[t.Dict[str, t.Any]] = None,
    ) -> httpx.Response:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            path_params=path_params,
            query_params=query_params,
            header_params=header_params,
            json_body=json_body,
        )

        response = httpx.request(**httpx_kwargs)
        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            time.sleep(sleep_time)
            httpx_kwargs["headers"]["svix-retry-count"] = str(retry_count)
            response = httpx.request(**httpx_kwargs)

        return _filter_response_for_errors_response(response)


def _filter_response_for_errors_response(response: httpx.Response) -> httpx.Response:
    if 200 <= response.status_code <= 299:
        return response

    if response.status_code == HTTPStatus.BAD_REQUEST:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.UNAUTHORIZED:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.FORBIDDEN:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.NOT_FOUND:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.CONFLICT:
        raise HttpError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.UNPROCESSABLE_ENTITY:
        raise HTTPValidationError.init_exception(response.json(), response.status_code)
    if response.status_code == HTTPStatus.TOO_MANY_REQUESTS:
        raise HttpError.init_exception(response.json(), response.status_code)
    raise UnexpectedStatus(response.status_code, response.content)
