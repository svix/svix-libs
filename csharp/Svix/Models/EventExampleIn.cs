// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventExampleIn
    {
        [JsonProperty("eventType", Required = Required.Always)]
        public string EventType { get; set; }

        [JsonProperty("exampleIndex")]
        public ulong? ExampleIndex { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventExampleIn {\n");
            sb.Append("  EventType: ").Append(EventType).Append('\n');
            sb.Append("  ExampleIndex: ").Append(ExampleIndex).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
