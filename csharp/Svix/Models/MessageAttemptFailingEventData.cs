// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event.
    /// <summary>
    public class MessageAttemptFailingEventData
    {
        [JsonProperty("appId", Required = Required.Always)]
        public string AppId { get; set; }

        [JsonProperty("appUid")]
        public string? AppUid { get; set; } = null;

        [JsonProperty("endpointId", Required = Required.Always)]
        public string EndpointId { get; set; }

        [JsonProperty("lastAttempt", Required = Required.Always)]
        public MessageAttemptFailedData LastAttempt { get; set; }

        [JsonProperty("msgEventId")]
        public string? MsgEventId { get; set; } = null;

        [JsonProperty("msgId", Required = Required.Always)]
        public string MsgId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptFailingEventData {\n");
            sb.Append("  AppId: ").Append(AppId).Append('\n');
            sb.Append("  AppUid: ").Append(AppUid).Append('\n');
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  LastAttempt: ").Append(LastAttempt).Append('\n');
            sb.Append("  MsgEventId: ").Append(MsgEventId).Append('\n');
            sb.Append("  MsgId: ").Append(MsgId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
