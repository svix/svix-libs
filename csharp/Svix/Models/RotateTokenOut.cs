// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RotateTokenOut
    {
        [JsonProperty("ingestUrl", Required = Required.Always)]
        public string IngestUrl { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RotateTokenOut {\n");
            sb.Append("  IngestUrl: ").Append(IngestUrl).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
