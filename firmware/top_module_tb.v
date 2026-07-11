`timescale 1ns/1ps
`include "top_module.v"

module top_module_tb;
    reg clk_50mhz;
    reg hardware_reset;
    reg network_line_in;
    reg [7:0] kinematic_target;

    wire protected_line_out;
    wire cpu_irq_pin;
    wire servo_pwm_pin;

    top_module uut (
        .CLK_50MHZ(clk_50mhz),
        .HARDWARE_RESET(hardware_reset),
        .NETWORK_LINE_IN(network_line_in),
        .KINEMATIC_TARGET(kinematic_target),
        .PROTECTED_LINE_OUT(protected_line_out),
        .CPU_IRQ_PIN(cpu_irq_pin),
        .SERVO_PWM_PIN(servo_pwm_pin)
    );

    always #10 clk_50mhz = ~clk_50mhz;

    initial begin
        $dumpfile("firmware/simulation.vcd");
        $dumpvars(0, top_module_tb);

        clk_50mhz = 0;
        hardware_reset = 1;
        network_line_in = 0;
        kinematic_target = 8'd90;

        #40;
        hardware_reset = 0;
        
        #100;
        kinematic_target = 8'd180;
        
        #200;
        network_line_in = 1;
        #40;
        network_line_in = 0;
        
        #500;
        network_line_in = 1;
        
        #1000;
        $finish;
    end
endmodule

