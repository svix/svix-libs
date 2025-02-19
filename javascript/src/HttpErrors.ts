export class HttpErrorOut {
  "code": string;
  "detail": string;

  static readonly discriminator: string | undefined = undefined;

  static readonly mapping: { [index: string]: string } | undefined = undefined;

  static readonly attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
    format: string;
  }> = [
    {
      name: "code",
      baseName: "code",
      type: "string",
      format: "",
    },
    {
      name: "detail",
      baseName: "detail",
      type: "string",
      format: "",
    },
  ];

  static getAttributeTypeMap() {
    return HttpErrorOut.attributeTypeMap;
  }
}

/**
 * Validation errors have their own schema to provide context for invalid requests eg. mismatched types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY error.
 */
export class ValidationError {
  /**
   * The location as a [`Vec`] of [`String`]s -- often in the form `[\"body\", \"field_name\"]`, `[\"query\", \"field_name\"]`, etc. They may, however, be arbitrarily deep.
   */
  "loc": Array<string>;
  /**
   * The message accompanying the validation error item.
   */
  "msg": string;
  /**
   * The type of error, often \"type_error\" or \"value_error\", but sometimes with more context like as \"value_error.number.not_ge\"
   */
  "type": string;

  static readonly discriminator: string | undefined = undefined;

  static readonly mapping: { [index: string]: string } | undefined = undefined;

  static readonly attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
    format: string;
  }> = [
    {
      name: "loc",
      baseName: "loc",
      type: "Array<string>",
      format: "",
    },
    {
      name: "msg",
      baseName: "msg",
      type: "string",
      format: "",
    },
    {
      name: "type",
      baseName: "type",
      type: "string",
      format: "",
    },
  ];

  static getAttributeTypeMap() {
    return ValidationError.attributeTypeMap;
  }
}

export class HTTPValidationError {
  "detail": Array<ValidationError>;

  static readonly discriminator: string | undefined = undefined;

  static readonly mapping: { [index: string]: string } | undefined = undefined;

  static readonly attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
    format: string;
  }> = [
    {
      name: "detail",
      baseName: "detail",
      type: "Array<ValidationError>",
      format: "",
    },
  ];

  static getAttributeTypeMap() {
    return HTTPValidationError.attributeTypeMap;
  }
}
