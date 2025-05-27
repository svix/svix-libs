// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when a background task is finished.
    /// <summary>
    public class BackgroundTaskFinishedEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public BackgroundTaskFinishedEvent2 Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class BackgroundTaskFinishedEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
