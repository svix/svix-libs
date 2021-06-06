/*
 * Svix
 * The Svix server API documentation
 *
 * The version of the OpenAPI document: 1.4
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


package svix.openapi.model;

import java.util.Objects;
import java.util.Arrays;
import com.google.gson.TypeAdapter;
import com.google.gson.annotations.JsonAdapter;
import com.google.gson.annotations.SerializedName;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonWriter;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import svix.openapi.model.EventTypeInOut;

/**
 * ListResponseEventTypeInOut
 */
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", date = "2021-06-06T18:08:35.083645+03:00[Asia/Jerusalem]")
public class ListResponseEventTypeInOut {
  public static final String SERIALIZED_NAME_DATA = "data";
  @SerializedName(SERIALIZED_NAME_DATA)
  private List<EventTypeInOut> data = new ArrayList<EventTypeInOut>();

  public static final String SERIALIZED_NAME_ITERATOR = "iterator";
  @SerializedName(SERIALIZED_NAME_ITERATOR)
  private String iterator;

  public static final String SERIALIZED_NAME_DONE = "done";
  @SerializedName(SERIALIZED_NAME_DONE)
  private Boolean done;


  public ListResponseEventTypeInOut data(List<EventTypeInOut> data) {
    
    this.data = data;
    return this;
  }

  public ListResponseEventTypeInOut addDataItem(EventTypeInOut dataItem) {
    this.data.add(dataItem);
    return this;
  }

   /**
   * Get data
   * @return data
  **/
  @ApiModelProperty(required = true, value = "")

  public List<EventTypeInOut> getData() {
    return data;
  }


  public void setData(List<EventTypeInOut> data) {
    this.data = data;
  }


  public ListResponseEventTypeInOut iterator(String iterator) {
    
    this.iterator = iterator;
    return this;
  }

   /**
   * Get iterator
   * @return iterator
  **/
  @javax.annotation.Nullable
  @ApiModelProperty(value = "")

  public String getIterator() {
    return iterator;
  }


  public void setIterator(String iterator) {
    this.iterator = iterator;
  }


  public ListResponseEventTypeInOut done(Boolean done) {
    
    this.done = done;
    return this;
  }

   /**
   * Get done
   * @return done
  **/
  @ApiModelProperty(required = true, value = "")

  public Boolean getDone() {
    return done;
  }


  public void setDone(Boolean done) {
    this.done = done;
  }


  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    ListResponseEventTypeInOut listResponseEventTypeInOut = (ListResponseEventTypeInOut) o;
    return Objects.equals(this.data, listResponseEventTypeInOut.data) &&
        Objects.equals(this.iterator, listResponseEventTypeInOut.iterator) &&
        Objects.equals(this.done, listResponseEventTypeInOut.done);
  }

  @Override
  public int hashCode() {
    return Objects.hash(data, iterator, done);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class ListResponseEventTypeInOut {\n");
    sb.append("    data: ").append(toIndentedString(data)).append("\n");
    sb.append("    iterator: ").append(toIndentedString(iterator)).append("\n");
    sb.append("    done: ").append(toIndentedString(done)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

}

