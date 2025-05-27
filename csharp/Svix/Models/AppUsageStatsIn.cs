// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppUsageStatsIn
    {
        [JsonProperty("appIds")]
        public List<string>? AppIds { get; set; } = null;

        [JsonProperty("since", Required = Required.Always)]
        public DateTime Since { get; set; }

        [JsonProperty("until", Required = Required.Always)]
        public DateTime Until { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AppUsageStatsIn {\n");
            sb.Append("  AppIds: ").Append(AppIds).Append('\n');
            sb.Append("  Since: ").Append(Since).Append('\n');
            sb.Append("  Until: ").Append(Until).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
