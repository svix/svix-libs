// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EventTypeUpdate
    {
        [JsonProperty("archived")]
        public bool? Archived { get; set; } = null;

        [JsonProperty("deprecated")]
        public bool? Deprecated { get; set; } = null;

        [JsonProperty("description", Required = Required.Always)]
        public string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("groupName")]
        public string? GroupName { get; set; } = null;

        [JsonProperty("schemas")]
        public Object? Schemas { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EventTypeUpdate {\n");
            sb.Append("  Archived: ").Append(Archived).Append('\n');
            sb.Append("  Deprecated: ").Append(Deprecated).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append('\n');
            sb.Append("  GroupName: ").Append(GroupName).Append('\n');
            sb.Append("  Schemas: ").Append(Schemas).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
