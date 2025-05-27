// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent on a successful dispatch after an earlier failure op webhook has already been sent.
    /// <summary>
    public class MessageAttemptRecoveredEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public MessageAttemptRecoveredEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptRecoveredEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
