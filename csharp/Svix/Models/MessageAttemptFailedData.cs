// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class MessageAttemptFailedData
    {
        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("responseStatusCode", Required = Required.Always)]
        public short ResponseStatusCode { get; set; }

        [JsonProperty("timestamp", Required = Required.Always)]
        public DateTime Timestamp { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class MessageAttemptFailedData {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  ResponseStatusCode: ").Append(ResponseStatusCode).Append('\n');
            sb.Append("  Timestamp: ").Append(Timestamp).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
