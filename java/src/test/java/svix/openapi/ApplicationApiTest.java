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


package svix.openapi;

import svix.ApiException;
import svix.openapi.model.ApplicationIn;
import svix.openapi.model.ApplicationOut;
import svix.openapi.model.HTTPValidationError;
import svix.openapi.model.HttpErrorOut;
import svix.openapi.model.ListResponseApplicationOut;
import org.junit.Test;
import org.junit.Ignore;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * API tests for ApplicationApi
 */
@Ignore
public class ApplicationApiTest {

    private final ApplicationApi api = new ApplicationApi();

    
    /**
     * Create Application
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void createApplicationApiV1AppPostTest() throws ApiException {
        ApplicationIn applicationIn = null;
        ApplicationOut response = api.createApplicationApiV1AppPost(applicationIn);

        // TODO: test validations
    }
    
    /**
     * Delete Application
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void deleteApplicationApiV1AppAppIdDeleteTest() throws ApiException {
        String appId = null;
        api.deleteApplicationApiV1AppAppIdDelete(appId);

        // TODO: test validations
    }
    
    /**
     * Get Application
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void getApplicationApiV1AppAppIdGetTest() throws ApiException {
        String appId = null;
        ApplicationOut response = api.getApplicationApiV1AppAppIdGet(appId);

        // TODO: test validations
    }
    
    /**
     * List Applications
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void listApplicationsApiV1AppGetTest() throws ApiException {
        String iterator = null;
        Integer limit = null;
        ListResponseApplicationOut response = api.listApplicationsApiV1AppGet(iterator, limit);

        // TODO: test validations
    }
    
    /**
     * Update Application
     *
     * 
     *
     * @throws ApiException
     *          if the Api call fails
     */
    @Test
    public void updateApplicationApiV1AppAppIdPutTest() throws ApiException {
        String appId = null;
        ApplicationIn applicationIn = null;
        ApplicationOut response = api.updateApplicationApiV1AppAppIdPut(appId, applicationIn);

        // TODO: test validations
    }
    
}
