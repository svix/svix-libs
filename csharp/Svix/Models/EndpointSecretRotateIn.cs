// this file is @generated
using System.Text.Json.Serialization;

namespace Svix.Models
{
    public class EndpointSecretRotateIn(string? key = null) : BaseModel
    {
        [JsonPropertyName("key")]
        public string? Key { get; set; } = key;
    }
}
