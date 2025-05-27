// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ConnectorOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("description", Required = Required.Always)]
        public string Description { get; set; }

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("instructions", Required = Required.Always)]
        public string Instructions { get; set; }

        [JsonProperty("instructionsLink")]
        public string? InstructionsLink { get; set; } = null;

        [JsonProperty("kind", Required = Required.Always)]
        public ConnectorKind Kind { get; set; }

        [JsonProperty("logo", Required = Required.Always)]
        public string Logo { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("orgId", Required = Required.Always)]
        public string OrgId { get; set; }

        [JsonProperty("transformation", Required = Required.Always)]
        public string Transformation { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ConnectorOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append('\n');
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Instructions: ").Append(Instructions).Append('\n');
            sb.Append("  InstructionsLink: ").Append(InstructionsLink).Append('\n');
            sb.Append("  Kind: ").Append(Kind).Append('\n');
            sb.Append("  Logo: ").Append(Logo).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  OrgId: ").Append(OrgId).Append('\n');
            sb.Append("  Transformation: ").Append(Transformation).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
