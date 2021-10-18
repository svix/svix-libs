# frozen_string_literal: true

module Svix
    class FetchOptionsMessageAttempt
        attr_reader :limit
        attr_reader :iterator

        def initialize(limit = 50, iterator = "")
            @limit=limit
            @iterator=iterator
        end
    end
end
