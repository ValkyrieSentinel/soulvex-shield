module kinematic_servo (
    input wire clk,
    input wire rst,
    input wire [7:0] target_pos,
    output reg pwm_pin
);
    reg [19:0] clk_ticks;
    reg [19:0] pulse_width;

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            clk_ticks   <= 20'd0;
            pulse_width <= 20'd0;
            pwm_pin     <= 1'b0;
        end else begin
            pulse_width <= 20'd50000 + ({12'b0, target_pos} * 20'd277);

            if (clk_ticks < 20'd1000000) begin
                clk_ticks <= clk_ticks + 1'b1;
            end else begin
                clk_ticks <= 20'd0;
            end

            pwm_pin <= (clk_ticks < pulse_width) ? 1'b1 : 1'b0;
        end
    end
endmodule

