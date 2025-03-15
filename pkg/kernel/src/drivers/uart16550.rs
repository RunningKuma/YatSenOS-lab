use core::fmt;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};
use bitflags::bitflags;



bitflags! {
  pub struct LCR: u8{
    const word_length_5 = 0b00000000;
    const word_length_6 = 0b00000001;
    const word_length_7 = 0b00000010;
    const word_length_8 = 0b00000011;
    const stop_bit_1 = 0b00000000;
    const stop_bit_2 = 0b00000100;
    const parity_none = 0b00000000;
    const parity_odd = 0b00001000;
    const parity_even = 0b00011000;
    const break_ctrl = 0b01000000;
    const dlab = 0b10000000;
  }
}
pub struct SerialPort<const BASE_ADDR: u16>{
    port_base_data: Port<u8>,
    interrupt_enable_register: PortWriteOnly<u8>,
    interrupt_identification_register: PortWriteOnly<u8>,
    line_control_register: PortWriteOnly<u8>,
    modem_control_register: PortWriteOnly<u8>,
    line_status_register: PortReadOnly<u8>,
}

impl<const BASE_ADDR: u16> SerialPort<BASE_ADDR> {
    pub const fn new() -> Self {
        Self {
            port_base_data: Port::new(BASE_ADDR),
            interrupt_enable_register: PortWriteOnly::new(BASE_ADDR + 1),
            interrupt_identification_register: PortWriteOnly::new(BASE_ADDR + 2),
            line_control_register: PortWriteOnly::new(BASE_ADDR + 3),
            modem_control_register: PortWriteOnly::new(BASE_ADDR + 4),
            line_status_register: PortReadOnly::new(BASE_ADDR + 5),
        }
    }

    /// Initializes the serial port.
    pub fn init(&mut self) {//should change into mut
        unsafe{
        // FIXME: Initialize the serial port
          self.interrupt_enable_register.write(0x00);// Disable all interrupts
          self.line_control_register.write(0x80);// Enable DLAB (set baud rate divisor)
          self.port_base_data.write(0x03); // Set divisor to 3 (lo byte) 38400 baud
          self.interrupt_enable_register.write(0x00); //                  (hi byte)
          self.line_control_register.write(0x03); // 8 bits, no parity, one stop bit
          self.interrupt_identification_register.write(0xC7);   // Enable FIFO, clear them, with 14-byte threshold
          self.modem_control_register.write(0x0B); // IRQs enabled, RTS/DSR set
          self.modem_control_register.write(0x1E); //set in loopback mode
          self.port_base_data.write(0xAE); //Test serial chip
          self.interrupt_enable_register.write(0x01); //Enable interrupts
        if self.port_base_data.read() != 0xAE {
            panic!("Serial port test failed");
        }
        self.modem_control_register.write(0x0F); // back to Normal option mode
        }   
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        unsafe{
        // FIXME: Send a byte on the serial port 
        let is_tran_empty = self.line_status_register.read() & 0x20;
        while is_tran_empty == 0 {
            // Wait for the transmit buffer to be empty
        }
        self.port_base_data.write(data);//Start 2 sent!
    }
    }

    /// Receives a byte on the serial port no wait.
    pub fn receive(&mut self) -> Option<u8> {
        unsafe{
        // FIXME: Receive a byte on the serial port no wait
        let serial_received = self.line_status_register.read() & 0x01;

        if serial_received != 0 {
            Some(self.port_base_data.read())//Start 2 received!
        }
        else{
            None // No data received
        }
    }
}  


}
impl<const BASE_ADDR:u16> fmt::Write for SerialPort<BASE_ADDR> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
    }

