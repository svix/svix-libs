// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when a message delivery has failed (all of the retry attempts have been exhausted).
    /// <summary>
    public class MessageAttemptExhaustedEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public MessageAttemptExhaustedEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptExhaustedEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
