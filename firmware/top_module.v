`include "hardware_firewall.v"
`include "kinematic_servo.v"

module top_module (
    input wire CLK_50MHZ,
    input wire HARDWARE_RESET,
    input wire NETWORK_LINE_IN,
    input wire [7:0] KINEMATIC_TARGET,
    output wire PROTECTED_LINE_OUT,
    output wire CPU_IRQ_PIN,
    output wire SERVO_PWM_PIN
);
    hardware_firewall firewall_inst (
        .clk(CLK_50MHZ),
        .rst(HARDWARE_RESET),
        .raw_signal(NETWORK_LINE_IN),
        .secure_signal(PROTECTED_LINE_OUT),
        .irq_trigger(CPU_IRQ_PIN)
    );

    kinematic_servo servo_inst (
        .clk(CLK_50MHZ),
        .rst(HARDWARE_RESET),
        .target_pos(KINEMATIC_TARGET),
        .pwm_pin(SERVO_PWM_PIN)
    );
endmodule
