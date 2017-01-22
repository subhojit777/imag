#!/usr/bin/env ruby

module RubyImag

  IMAG_INIT_FN_NAME = 'imag_ruby_initialize'

  def self.setup binary_path
    require 'fiddle'

    self.core_setup binary_path
    self.classes_setup
  end

  module Logger

    def self.init debug, verbose, color
      Imag.init_logger debug, verbose, color
    end

    def self.trace msg
      Imag.trace msg
    end

    def self.dbg msg
      Imag.dbg msg
    end

    def self.debug msg
      Imag.debug msg
    end

    def self.info msg
      Imag.info msg
    end

    def self.warn msg
      Imag.warn msg
    end

    def self.error msg
      Imag.error msg
    end

  end

  private

  def self.class_names
    [
      :StoreId             ,
      :StoreHandle         ,
      :FileLockEntryHandle ,
      :EntryHeader         ,
      :EntryContent        ,
    ]
  end

  def self.core_setup binary
    lib = Fiddle::dlopen binary
    Fiddle::Function::new(lib[RubyImag::IMAG_INIT_FN_NAME], [], Fiddle::TYPE_VOIDP).call

    self.class_names.map {|n| [n, "R#{n}".to_sym ] }.each do |elem|
      RubyImag.const_set elem.first, Kernel.const_get(elem.last)
    end
  end

  def self.classes_setup
    self.class_storeid_setup
  end

  def self.class_storeid_setup
    RubyImag::StoreId.class_exec do
      def to_s
        self.to_str
      end
    end
  end

end

if __FILE__ == $0
  puts "Running some tests..."
  puts "I hope you passed the library object as first argument..."
  begin
    RubyImag.setup ARGV.first
  rescue Exception => e
    puts "Seems not to be the case... or something else went wrong..."
    puts e
    exit 1
  end

  RubyImag::Logger.init true, true, true
  RubyImag::Logger.info "The Logger should work now"

  RubyImag::Logger.info "Lets see whether we have properly setup StoreId"
  RubyImag::Logger.info RubyImag::StoreId::new_baseless("baselessId").to_s
  RubyImag::Logger.info "Seems good."
end

