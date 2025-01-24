import asyncio
import datetime
import random
from http import HTTPStatus
from time import sleep
from typing import Any, Dict, List, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.http_error import HttpError
from ...models.http_validation_error import HTTPValidationError
from ...models.list_response_message_attempt_out import ListResponseMessageAttemptOut
from ...models.message_status import MessageStatus
from ...models.status_code_class import StatusCodeClass
from ...types import UNSET, Response, Unset


def _get_kwargs(
    app_id: str,
    msg_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    endpoint_id: Union[Unset, None, str] = UNSET,
    channel: Union[Unset, None, str] = UNSET,
    tag: Union[Unset, None, str] = UNSET,
    status: Union[Unset, None, MessageStatus] = UNSET,
    before: Union[Unset, None, datetime.datetime] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
    status_code_class: Union[Unset, None, StatusCodeClass] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
) -> Dict[str, Any]:
    url = "{}/api/v1/app/{app_id}/msg/{msg_id}/attempt".format(client.base_url, app_id=app_id, msg_id=msg_id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    params: Dict[str, Any] = {}
    params["limit"] = limit

    params["iterator"] = iterator

    params["endpoint_id"] = endpoint_id

    params["channel"] = channel

    params["tag"] = tag

    json_status: Union[Unset, None, int] = UNSET
    if not isinstance(status, Unset):
        json_status = status.value if status else None

    params["status"] = json_status

    json_before: Union[Unset, None, str] = UNSET
    if not isinstance(before, Unset):
        json_before = before.isoformat() if before else None

    params["before"] = json_before

    json_after: Union[Unset, None, str] = UNSET
    if not isinstance(after, Unset):
        json_after = after.isoformat() if after else None

    params["after"] = json_after

    json_status_code_class: Union[Unset, None, int] = UNSET
    if not isinstance(status_code_class, Unset):
        json_status_code_class = status_code_class.value if status_code_class else None

    params["status_code_class"] = json_status_code_class

    json_event_types: Union[Unset, None, List[str]] = UNSET
    if not isinstance(event_types, Unset):
        if event_types is None:
            json_event_types = None
        else:
            json_event_types = event_types

    params["event_types"] = json_event_types

    params = {k: v for k, v in params.items() if v is not UNSET and v is not None}

    return {
        "method": "get",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "follow_redirects": client.follow_redirects,
        "params": params,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> ListResponseMessageAttemptOut:
    if response.status_code == HTTPStatus.OK:
        response_200 = ListResponseMessageAttemptOut.from_dict(response.json())

        return response_200
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


def _build_response(*, client: Client, response: httpx.Response) -> Response[ListResponseMessageAttemptOut]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def request_sync_detailed(
    app_id: str,
    msg_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    endpoint_id: Union[Unset, None, str] = UNSET,
    channel: Union[Unset, None, str] = UNSET,
    tag: Union[Unset, None, str] = UNSET,
    status: Union[Unset, None, MessageStatus] = UNSET,
    before: Union[Unset, None, datetime.datetime] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
    status_code_class: Union[Unset, None, StatusCodeClass] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
) -> Response[ListResponseMessageAttemptOut]:
    r"""List Attempts

     Deprecated: Please use \"List Attempts by Endpoint\" and \"List Attempts by Msg\" instead.

    Note that by default this endpoint is limited to retrieving 90 days' worth of data
    relative to now or, if an iterator is provided, 90 days before/after the time indicated
    by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    set the `before` or `after` parameter as appropriate.

    `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        endpoint_id (Union[Unset, None, str]): Filter the attempts based on the attempted endpoint
            Example: unique-ep-identifier.
        channel (Union[Unset, None, str]): Filter response based on the channel Example:
            project_1337.
        tag (Union[Unset, None, str]): Filter response based on the tag Example: project_1337.
        status (Union[Unset, None, MessageStatus]): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        before (Union[Unset, None, datetime.datetime]): Only include items created before a
            certain date
        after (Union[Unset, None, datetime.datetime]): Only include items created after a certain
            date
        status_code_class (Union[Unset, None, StatusCodeClass]): The different classes of HTTP
            status codes:
            - CodeNone = 0
            - Code1xx = 100
            - Code2xx = 200
            - Code3xx = 300
            - Code4xx = 400
            - Code5xx = 500
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ListResponseMessageAttemptOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        msg_id=msg_id,
        client=client,
        limit=limit,
        iterator=iterator,
        endpoint_id=endpoint_id,
        channel=channel,
        tag=tag,
        status=status,
        before=before,
        after=after,
        status_code_class=status_code_class,
        event_types=event_types,
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
    msg_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    endpoint_id: Union[Unset, None, str] = UNSET,
    channel: Union[Unset, None, str] = UNSET,
    tag: Union[Unset, None, str] = UNSET,
    status: Union[Unset, None, MessageStatus] = UNSET,
    before: Union[Unset, None, datetime.datetime] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
    status_code_class: Union[Unset, None, StatusCodeClass] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
) -> ListResponseMessageAttemptOut:
    r"""List Attempts

     Deprecated: Please use \"List Attempts by Endpoint\" and \"List Attempts by Msg\" instead.

    Note that by default this endpoint is limited to retrieving 90 days' worth of data
    relative to now or, if an iterator is provided, 90 days before/after the time indicated
    by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    set the `before` or `after` parameter as appropriate.

    `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        endpoint_id (Union[Unset, None, str]): Filter the attempts based on the attempted endpoint
            Example: unique-ep-identifier.
        channel (Union[Unset, None, str]): Filter response based on the channel Example:
            project_1337.
        tag (Union[Unset, None, str]): Filter response based on the tag Example: project_1337.
        status (Union[Unset, None, MessageStatus]): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        before (Union[Unset, None, datetime.datetime]): Only include items created before a
            certain date
        after (Union[Unset, None, datetime.datetime]): Only include items created after a certain
            date
        status_code_class (Union[Unset, None, StatusCodeClass]): The different classes of HTTP
            status codes:
            - CodeNone = 0
            - Code1xx = 100
            - Code2xx = 200
            - Code3xx = 300
            - Code4xx = 400
            - Code5xx = 500
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ListResponseMessageAttemptOut
    """

    return request_sync_detailed(
        app_id=app_id,
        msg_id=msg_id,
        client=client,
        limit=limit,
        iterator=iterator,
        endpoint_id=endpoint_id,
        channel=channel,
        tag=tag,
        status=status,
        before=before,
        after=after,
        status_code_class=status_code_class,
        event_types=event_types,
    ).parsed


async def request_asyncio_detailed(
    app_id: str,
    msg_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    endpoint_id: Union[Unset, None, str] = UNSET,
    channel: Union[Unset, None, str] = UNSET,
    tag: Union[Unset, None, str] = UNSET,
    status: Union[Unset, None, MessageStatus] = UNSET,
    before: Union[Unset, None, datetime.datetime] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
    status_code_class: Union[Unset, None, StatusCodeClass] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
) -> Response[ListResponseMessageAttemptOut]:
    r"""List Attempts

     Deprecated: Please use \"List Attempts by Endpoint\" and \"List Attempts by Msg\" instead.

    Note that by default this endpoint is limited to retrieving 90 days' worth of data
    relative to now or, if an iterator is provided, 90 days before/after the time indicated
    by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    set the `before` or `after` parameter as appropriate.

    `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        endpoint_id (Union[Unset, None, str]): Filter the attempts based on the attempted endpoint
            Example: unique-ep-identifier.
        channel (Union[Unset, None, str]): Filter response based on the channel Example:
            project_1337.
        tag (Union[Unset, None, str]): Filter response based on the tag Example: project_1337.
        status (Union[Unset, None, MessageStatus]): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        before (Union[Unset, None, datetime.datetime]): Only include items created before a
            certain date
        after (Union[Unset, None, datetime.datetime]): Only include items created after a certain
            date
        status_code_class (Union[Unset, None, StatusCodeClass]): The different classes of HTTP
            status codes:
            - CodeNone = 0
            - Code1xx = 100
            - Code2xx = 200
            - Code3xx = 300
            - Code4xx = 400
            - Code5xx = 500
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[ListResponseMessageAttemptOut]
    """

    kwargs = _get_kwargs(
        app_id=app_id,
        msg_id=msg_id,
        client=client,
        limit=limit,
        iterator=iterator,
        endpoint_id=endpoint_id,
        channel=channel,
        tag=tag,
        status=status,
        before=before,
        after=after,
        status_code_class=status_code_class,
        event_types=event_types,
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
    msg_id: str,
    *,
    client: AuthenticatedClient,
    limit: Union[Unset, None, int] = UNSET,
    iterator: Union[Unset, None, str] = UNSET,
    endpoint_id: Union[Unset, None, str] = UNSET,
    channel: Union[Unset, None, str] = UNSET,
    tag: Union[Unset, None, str] = UNSET,
    status: Union[Unset, None, MessageStatus] = UNSET,
    before: Union[Unset, None, datetime.datetime] = UNSET,
    after: Union[Unset, None, datetime.datetime] = UNSET,
    status_code_class: Union[Unset, None, StatusCodeClass] = UNSET,
    event_types: Union[Unset, None, List[str]] = UNSET,
) -> ListResponseMessageAttemptOut:
    r"""List Attempts

     Deprecated: Please use \"List Attempts by Endpoint\" and \"List Attempts by Msg\" instead.

    Note that by default this endpoint is limited to retrieving 90 days' worth of data
    relative to now or, if an iterator is provided, 90 days before/after the time indicated
    by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    set the `before` or `after` parameter as appropriate.

    `msg_id`: Use a message id or a message `eventId`

    Args:
        app_id (str): The app's ID or UID Example: unique-app-identifier.
        msg_id (str): The msg's ID or UID Example: unique-msg-identifier.
        limit (Union[Unset, None, int]): Limit the number of returned items
        iterator (Union[Unset, None, str]): The iterator returned from a prior invocation Example:
            atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2.
        endpoint_id (Union[Unset, None, str]): Filter the attempts based on the attempted endpoint
            Example: unique-ep-identifier.
        channel (Union[Unset, None, str]): Filter response based on the channel Example:
            project_1337.
        tag (Union[Unset, None, str]): Filter response based on the tag Example: project_1337.
        status (Union[Unset, None, MessageStatus]): The sending status of the message:
            - Success = 0
            - Pending = 1
            - Fail = 2
            - Sending = 3
        before (Union[Unset, None, datetime.datetime]): Only include items created before a
            certain date
        after (Union[Unset, None, datetime.datetime]): Only include items created after a certain
            date
        status_code_class (Union[Unset, None, StatusCodeClass]): The different classes of HTTP
            status codes:
            - CodeNone = 0
            - Code1xx = 100
            - Code2xx = 200
            - Code3xx = 300
            - Code4xx = 400
            - Code5xx = 500
        event_types (Union[Unset, None, List[str]]): Filter response based on the event type

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        ListResponseMessageAttemptOut
    """

    return (
        await request_asyncio_detailed(
            app_id=app_id,
            msg_id=msg_id,
            client=client,
            limit=limit,
            iterator=iterator,
            endpoint_id=endpoint_id,
            channel=channel,
            tag=tag,
            status=status,
            before=before,
            after=after,
            status_code_class=status_code_class,
            event_types=event_types,
        )
    ).parsed
