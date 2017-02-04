# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'imag/version'

Gem::Specification.new do |spec|
  spec.name          = "imag"
  spec.version       = Imag::VERSION
  spec.authors       = ["Matthias Beyer"]
  spec.email         = ["mail@beyermatthias.de"]

  spec.summary       = %q{A Ruby gem to script imag.}
  spec.description   = %q{A Ruby gem to script imag, the personal information management suite for the commandline}
  spec.homepage      = "http://imag-pim.org"

  spec.files         = `git ls-files -z`.split("\x0").reject do |f|
    f.match(%r{^(test|spec|features)/})
  end

  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_development_dependency "bundler", "~> 1.13"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency 'thermite', "~> 0.11", ">= 0.11.1"

  spec.extensions << 'ext/Rakefile'
end
