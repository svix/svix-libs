// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class RecoverOut
    {
        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("status", Required = Required.Always)]
        public BackgroundTaskStatus Status { get; set; }

        [JsonProperty("task", Required = Required.Always)]
        public BackgroundTaskType Task { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class RecoverOut {\n");
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Status: ").Append(Status).Append('\n');
            sb.Append("  Task: ").Append(Task).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
