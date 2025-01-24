# TODO - remove this special case when we fix the generated code for empty openapi structs
from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="AzureBlobStorageConfig")


@attr.s(auto_attribs=True)
class AzureBlobStorageConfig:
    """
    Attributes:
        access_key (str):
        account (str):
        container (str):
    """

    access_key: str
    account: str
    container: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        access_key = self.access_key
        account = self.account
        container = self.container

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "accessKey": access_key,
                "account": account,
                "container": container,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        access_key = d.pop("accessKey")

        account = d.pop("account")

        container = d.pop("container")

        azure_blob_storage_config = cls(
            access_key=access_key,
            account=account,
            container=container,
        )

        azure_blob_storage_config.additional_properties = d
        return azure_blob_storage_config

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
