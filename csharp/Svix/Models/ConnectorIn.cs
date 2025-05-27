// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ConnectorIn
    {
        [JsonProperty("description")]
        public string? Description { get; set; } = null;

        [JsonProperty("featureFlag")]
        public string? FeatureFlag { get; set; } = null;

        [JsonProperty("filterTypes")]
        public List<string>? FilterTypes { get; set; } = null;

        [JsonProperty("instructions")]
        public string? Instructions { get; set; } = null;

        [JsonProperty("instructionsLink")]
        public string? InstructionsLink { get; set; } = null;

        [JsonProperty("kind")]
        public ConnectorKind? Kind { get; set; } = null;

        [JsonProperty("logo", Required = Required.Always)]
        public string Logo { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("transformation", Required = Required.Always)]
        public string Transformation { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ConnectorIn {\n");
            sb.Append("  Description: ").Append(Description).Append('\n');
            sb.Append("  FeatureFlag: ").Append(FeatureFlag).Append('\n');
            sb.Append("  FilterTypes: ").Append(FilterTypes).Append('\n');
            sb.Append("  Instructions: ").Append(Instructions).Append('\n');
            sb.Append("  InstructionsLink: ").Append(InstructionsLink).Append('\n');
            sb.Append("  Kind: ").Append(Kind).Append('\n');
            sb.Append("  Logo: ").Append(Logo).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Transformation: ").Append(Transformation).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
