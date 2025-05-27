// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageAttemptOut
    {
        [JsonProperty("endpointId", Required = Required.Always)]
        public string EndpointId { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("msg")]
        public MessageOut? Msg { get; set; } = null;

        [JsonProperty("msgId", Required = Required.Always)]
        public string MsgId { get; set; }

        [JsonProperty("response", Required = Required.Always)]
        public string Response { get; set; }

        [JsonProperty("responseDurationMs", Required = Required.Always)]
        public long ResponseDurationMs { get; set; }

        [JsonProperty("responseStatusCode", Required = Required.Always)]
        public short ResponseStatusCode { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public MessageStatus Status { get; set; }

        [JsonProperty("timestamp", Required = Required.Always)]
        public DateTime Timestamp { get; set; }

        [JsonProperty("triggerType", Required = Required.Always)]
        public MessageAttemptTriggerType TriggerType { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptOut {\n");
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Msg: ").Append(Msg).Append('\n');
            sb.Append("  MsgId: ").Append(MsgId).Append('\n');
            sb.Append("  Response: ").Append(Response).Append('\n');
            sb.Append("  ResponseDurationMs: ").Append(ResponseDurationMs).Append('\n');
            sb.Append("  ResponseStatusCode: ").Append(ResponseStatusCode).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Timestamp: ").Append(Timestamp).Append('\n');
            sb.Append("  TriggerType: ").Append(TriggerType).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
