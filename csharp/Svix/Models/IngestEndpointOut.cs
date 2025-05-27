// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IngestEndpointOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public string Description { get; set; }

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("metadata", Required = Required.Always)]
        public Dictionary<string, string> Metadata { get; set; }

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  Disabled: ").Append(Disabled).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  RateLimit: ").Append(RateLimit).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
