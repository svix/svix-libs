// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ReplayIn
    {
        [JsonProperty("since", Required = Required.Always)]
        public DateTime Since { get; set; }

        [JsonProperty("until")]
        public DateTime? Until { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ReplayIn {\n");
            sb.Append("  Since: ").Append(Since).Append('\n');
            sb.Append("  Until: ").Append(Until).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
