// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// The MessageOut equivalent of polling endpoint
    /// <summary>
    public class PollingEndpointMessageOut
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public string EventType { get; set; }

        [JsonProperty("headers")]
        public Dictionary<string, string>? Headers { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("payload", Required = Required.Always)]
        public Object Payload { get; set; }

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("timestamp", Required = Required.Always)]
        public DateTime Timestamp { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PollingEndpointMessageOut {\n");
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  EventId: ").Append(EventId).Append('\n');
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  Headers: ").Append(Headers).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("  Tags: ").Append(Tags).Append('\n');
            sb.Append("  Timestamp: ").Append(Timestamp).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
