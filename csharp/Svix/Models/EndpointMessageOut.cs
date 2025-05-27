// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// A model containing information on a given message plus additional fields on the last attempt for that message.
    /// <summary>
    public class EndpointMessageOut
    {
        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public string EventType { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("nextAttempt")]
        public DateTime? NextAttempt { get; set; } = null;

        [JsonProperty("payload", Required = Required.Always)]
        public Object Payload { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public MessageStatus Status { get; set; }

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("timestamp", Required = Required.Always)]
        public DateTime Timestamp { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointMessageOut {\n");
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  EventId: ").Append(EventId).Append('\n');
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  NextAttempt: ").Append(NextAttempt).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Tags: ").Append(Tags).Append('\n');
            sb.Append("  Timestamp: ").Append(Timestamp).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
