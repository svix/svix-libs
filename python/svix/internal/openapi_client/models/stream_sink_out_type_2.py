# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar

import attr

from ..models.stream_sink_out_type_2_type import StreamSinkOutType2Type

if TYPE_CHECKING:
    from ..models.sink_http_config import SinkHttpConfig


T = TypeVar("T", bound="StreamSinkOutType2")


@attr.s(auto_attribs=True)
class StreamSinkOutType2:
    """
    Attributes:
        config (SinkHttpConfig):
        type (StreamSinkOutType2Type):
    """

    config: "SinkHttpConfig"
    type: StreamSinkOutType2Type
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        config = self.config.to_dict()

        type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "config": config,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.sink_http_config import SinkHttpConfig

        d = src_dict.copy()
        config = SinkHttpConfig.from_dict(d.pop("config"))

        type = StreamSinkOutType2Type(d.pop("type"))

        stream_sink_out_type_2 = cls(
            config=config,
            type=type,
        )

        stream_sink_out_type_2.additional_properties = d
        return stream_sink_out_type_2

    @property
    def additional_keys(self) -> List[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
