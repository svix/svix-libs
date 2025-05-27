// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeImportOpenApiOut
    {
        [JsonProperty("data", Required = Required.Always)]
        public EventTypeImportOpenApiOutData Data { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventTypeImportOpenApiOut {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
