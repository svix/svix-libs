import asyncio
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.endpoint_out import EndpointOut
from ...models.endpoint_update import EndpointUpdate
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...types import Response


def _get_kwargs(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: EndpointUpdate,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/endpoint/{endpoint_id}".format(
        client.base_url, app_id=app_id, endpoint_id=endpoint_id
    )

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    json_json_body = json_body.to_dict()

    return {
        "method": "put",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "json": json_json_body,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> EndpointOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = EndpointOut.from_dict(response.json())

        return response_200
    if response.status_code == HTTPStatus.CREATED:
        response_201 = EndpointOut.from_dict(response.json())

        return response_201
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
    raise errors.UnexpectedStatus(response.status_code, response.content)


def _build_response(*, client: Client, response: httpx.Response) -> Response[EndpointOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: EndpointUpdate,
) -> Response[EndpointOut]:
    """Update Endpoint

     Update an endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (EndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EndpointOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        json_body=json_body,
    )

    kwargs["headers"] = {"svix-req-id": f"{random.getrandbits(32)}", **kwargs["headers"]}

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )
    for retry_count, sleep_time in enumerate(client.retry_schedule):
        if response.status_code < 500:
            break

        sleep(sleep_time)
        kwargs["headers"]["svix-retry-count"] = str(retry_count)
        response = httpx.request(
            verify=client.verify_ssl,
            **kwargs,
        )

    return _build_response(client=client, response=response)


def request_sync(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: EndpointUpdate,
) -> EndpointOut:
    """Update Endpoint

     Update an endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (EndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EndpointOut
    """

    return request_sync_detailed(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        json_body=json_body,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: EndpointUpdate,
) -> Response[EndpointOut]:
    """Update Endpoint

     Update an endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (EndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[EndpointOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        endpoint_id=endpoint_id,
        client=client,
        json_body=json_body,
    )

    kwargs["headers"] = {"svix-req-id": f"{random.getrandbits(32)}", **kwargs["headers"]}

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

        for retry_count, sleep_time in enumerate(client.retry_schedule):
            if response.status_code < 500:
                break

            await asyncio.sleep(sleep_time)
            kwargs["headers"]["svix-retry-count"] = str(retry_count)
            response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)


async def request_asyncio(
    app_id: str,
    endpoint_id: str,
    *,
    client: AuthenticatedClient,
    json_body: EndpointUpdate,
) -> EndpointOut:
    """Update Endpoint

     Update an endpoint.

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        endpoint_id (str): The ep's ID or UID Example: unique-ep-identifier.
        json_body (EndpointUpdate):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        EndpointOut
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            endpoint_id=endpoint_id,
            client=client,
            json_body=json_body,
        )
    ).parsed
