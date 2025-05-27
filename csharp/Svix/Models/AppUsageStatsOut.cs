// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppUsageStatsOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task", Required = Required.Always)]
        public BackgroundTaskType Task { get; set; }

        [JsonProperty("unresolvedAppIds", Required = Required.Always)]
        public List<string> UnresolvedAppIds { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AppUsageStatsOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Task: ").Append(Task).Append('\n');
            sb.Append("  UnresolvedAppIds: ").Append(UnresolvedAppIds).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
