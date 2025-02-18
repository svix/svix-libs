// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.AppPortalAccessIn
import com.svix.kotlin.models.AppPortalAccessOut
import com.svix.kotlin.models.ApplicationTokenExpireIn
import com.svix.kotlin.models.DashboardAccessOut
import okhttp3.Headers

data class AuthenticationAppPortalAccessOptions(val idempotencyKey: String? = null)

data class AuthenticationExpireAllOptions(val idempotencyKey: String? = null)

data class AuthenticationDashboardAccessOptions(val idempotencyKey: String? = null)

data class AuthenticationLogoutOptions(val idempotencyKey: String? = null)

class Authentication(private val client: SvixHttpClient) {

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to
     * the Consumer Application Portal.
     */
    suspend fun appPortalAccess(
        appId: String,
        appPortalAccessIn: AppPortalAccessIn,
        options: AuthenticationAppPortalAccessOptions = AuthenticationAppPortalAccessOptions(),
    ): AppPortalAccessOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/auth/app-portal-access/$appId")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<AppPortalAccessIn, AppPortalAccessOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = appPortalAccessIn,
        )
    }

    /** Expire all of the tokens associated with a specific application. */
    suspend fun expireAll(
        appId: String,
        applicationTokenExpireIn: ApplicationTokenExpireIn,
        options: AuthenticationExpireAllOptions = AuthenticationExpireAllOptions(),
    ) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/auth/app/$appId/expire-all")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<ApplicationTokenExpireIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = applicationTokenExpireIn,
        )
    }

    /**
     * DEPRECATED: Please use `app-portal-access` instead.
     *
     * Use this function to get magic links (and authentication codes) for connecting your users to
     * the Consumer Application Portal.
     *
     * @deprecated
     */
    @Deprecated("")
    suspend fun dashboardAccess(
        appId: String,
        options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
    ): DashboardAccessOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/auth/dashboard-access/$appId")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        return client.executeRequest<Any, DashboardAccessOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }

    /**
     * Logout an app token.
     *
     * Trying to log out other tokens will fail.
     */
    suspend fun logout(options: AuthenticationLogoutOptions = AuthenticationLogoutOptions()) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/auth/logout")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        client.executeRequest<Any, Boolean>("POST", url.build(), headers = headers.build())
    }
}
