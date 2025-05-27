// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageIn
    {
        [JsonProperty("application")]
        public ApplicationIn? Application { get; set; } = null;

        [JsonProperty("channels")]
        public List<string>? Channels { get; set; } = null;

        [JsonProperty("eventId")]
        public string? EventId { get; set; } = null;

        [JsonProperty("eventType", Required = Required.Always)]
        public string EventType { get; set; }

        [JsonProperty("payload", Required = Required.Always)]
        public Object Payload { get; set; }

        [JsonProperty("payloadRetentionHours")]
        public long? PayloadRetentionHours { get; set; } = null;

        [JsonProperty("payloadRetentionPeriod")]
        public long? PayloadRetentionPeriod { get; set; } = null;

        [JsonProperty("tags")]
        public List<string>? Tags { get; set; } = null;

        [JsonProperty("transformationsParams")]
        public Object? TransformationsParams { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageIn {\n");
            sb.Append("  Application: ").Append(Application).Append('\n');
            sb.Append("  Channels: ").Append(Channels).Append('\n');
            sb.Append("  EventId: ").Append(EventId).Append('\n');
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("  PayloadRetentionHours: ").Append(PayloadRetentionHours).Append('\n');
            sb.Append("  PayloadRetentionPeriod: ").Append(PayloadRetentionPeriod).Append('\n');
            sb.Append("  Tags: ").Append(Tags).Append('\n');
            sb.Append("  TransformationsParams: ").Append(TransformationsParams).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
