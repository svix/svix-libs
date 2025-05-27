// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOutData
    {
        [JsonProperty("modified", Required = Required.Always)]
        public List<string> Modified { get; set; }

        [JsonProperty("to_modify")]
        public List<EventTypeFromOpenApi>? ToModify { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventTypeImportOpenApiOutData {\n");
            sb.Append("  Modified: ").Append(Modified).Append('\n');
            sb.Append("  ToModify: ").Append(ToModify).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
