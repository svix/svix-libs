# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel


class ReplayIn(SvixBaseModel):
    since: datetime

    until: t.Optional[datetime] = None
