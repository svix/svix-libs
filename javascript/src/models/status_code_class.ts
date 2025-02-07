// this file is @generated
/**
 * The different classes of HTTP status codes:
 * - CodeNone = 0
 * - Code1xx = 100
 * - Code2xx = 200
 * - Code3xx = 300
 * - Code4xx = 400
 * - Code5xx = 500
 */
export enum StatusCodeClass {
  CodeNone = 0,
  Code1xx = 100,
  Code2xx = 200,
  Code3xx = 300,
  Code4xx = 400,
  Code5xx = 500,
}

export namespace StatusCodeClassUtil {
  export function _fromJsonObject(object: any): StatusCodeClass {
    return object;
  }

  export function _toJsonObject(self: StatusCodeClass): any {
    return self;
  }
}
