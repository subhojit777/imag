#!/usr/bin/env ruby

module Imag

  IMAG_INIT_FN_NAME = 'imag_ruby_initialize'

  def self.setup binary_path
    require binary_path

    self.core_setup
    self.classes_setup
  end

  module Logger

    def self.init debug, verbose, color
      RImag.init_logger debug, verbose, color
    end

    def self.trace msg
      RImag.trace msg
    end

    def self.dbg msg
      RImag.dbg msg
    end

    def self.debug msg
      RImag.debug msg
    end

    def self.info msg
      RImag.info msg
    end

    def self.warn msg
      RImag.warn msg
    end

    def self.error msg
      RImag.error msg
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

  def self.core_setup
    self.class_names.map {|n| [n, "R#{n}".to_sym ] }.each do |elem|
      Imag.const_set elem.first, Kernel.const_get(elem.last)
    end
  end

  def self.classes_setup
    self.class_storeid_setup
  end

  def self.class_storeid_setup
    Imag::StoreId.class_exec do
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
    Imag.setup ARGV.first
  rescue Exception => e
    puts "Seems not to be the case... or something else went wrong..."
    puts e
    exit 1
  end

  Imag::Logger.init true, true, true
  Imag::Logger.info "The Logger should work now"

  Imag::Logger.info "Lets see whether we have properly setup StoreId"
  Imag::Logger.info Imag::StoreId::new_baseless("baselessId").to_s
  Imag::Logger.info "Seems good."
end

