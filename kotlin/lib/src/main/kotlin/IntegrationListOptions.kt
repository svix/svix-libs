package com.svix.kotlin

class IntegrationListOptions : ListOptions() {
    override fun iterator(iterator: String) = apply { super.iterator(iterator) }

    override fun limit(limit: Int) = apply { super.limit(limit) }
}
