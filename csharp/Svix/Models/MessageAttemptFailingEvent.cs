// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent after a message has been failing for a few times.
    /// It's sent on the fourth failure. It complements `message.attempt.exhausted` which is sent after the last failure.
    /// <summary>
    public class MessageAttemptFailingEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public MessageAttemptFailingEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptFailingEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
