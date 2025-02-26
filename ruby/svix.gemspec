
lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "svix/version"

Gem::Specification.new do |spec|
  spec.name          = "svix"
  spec.version       = Svix::VERSION
  spec.authors       = ["Svix"]
  spec.email         = ["support@svix.com"]
  spec.license       = "MIT"

  spec.summary       = "Svix webhooks API client and webhook verification library"
  spec.description   = "Svix makes webhooks easy and reliable.  " \
                       "Learn more at https://www.svix.com"
  spec.homepage      = "https://www.svix.com"

  spec.post_install_message = <<~MESSAGE

    Thank you for installing svix!

    If you are using svix to send outgoing webhhooks, you should also install the `typhoeus` gem.
    If you are only using svix to consume incoming webhooks, you do not need it.

      gem 'typhoeus`

  MESSAGE

  # Prevent pushing this gem to RubyGems.org. To allow pushes either set the 'allowed_push_host'
  # to allow pushing to a single host or delete this section to allow pushing to any host.
  if spec.respond_to?(:metadata)
    spec.metadata["allowed_push_host"] = "https://rubygems.org"

    spec.metadata["homepage_uri"] = spec.homepage
    spec.metadata["source_code_uri"] = "https://github.com/svix/svix-libs"
    spec.metadata["changelog_uri"] = "https://github.com/svix/svix-libs/blob/main/ChangeLog.md"
  else
    raise "RubyGems 2.0 or newer is required to protect against " \
      "public gem pushes."
  end

  # Specify which files should be added to the gem when it is released.
  ignored = Regexp.union(
    /\A\.openapi-generator/,
    /\Aspec/,
    /\Apkg/,
    /\Atemplates/,
    /\A.gitignore/,
    /\A.openapi-generator-ignore/,
    /\Aopenapi-generator-config.json/,
    /.gem\z/
  )
  spec.files = Dir['**/*'].reject {|f| !File.file?(f) || ignored.match(f) }

  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rspec", "~> 3.2"
end
