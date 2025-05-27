// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageEndpointOut
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public string Description { get; set; }

        [JsonProperty("disabled")]
        public bool? Disabled { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("nextAttempt")]
        public DateTime? NextAttempt { get; set; } = null;

        [JsonProperty("rateLimit")]
        public ushort? RateLimit { get; set; } = null;

        [JsonProperty("status", Required = Required.Always)]
        public MessageStatus Status { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public string Url { get; set; }

        [JsonProperty("version", Required = Required.Always)]
        public int Version { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageEndpointOut {\n");
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  Disabled: ").Append(Disabled).Append('\n');
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  NextAttempt: ").Append(NextAttempt).Append('\n');
            sb.Append("  RateLimit: ").Append(RateLimit).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("  Version: ").Append(Version).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
