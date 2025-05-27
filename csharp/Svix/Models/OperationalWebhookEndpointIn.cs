// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class OperationalWebhookEndpointIn
    {
        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("metadata")]
        public Dictionary<string, string>? Metadata { get; set; } = null;

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("secret")]
        public string? Secret { get; set; } = null;

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("url", Required = Required.Always)]
        public string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class OperationalWebhookEndpointIn {\n");
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  Disabled: ").Append(Disabled).Append('\n');
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append('\n');
            sb.Append("  Metadata: ").Append(Metadata).Append('\n');
            sb.Append("  RateLimit: ").Append(RateLimit).Append('\n');
            sb.Append("  Secret: ").Append(Secret).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
