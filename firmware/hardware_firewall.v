module hardware_firewall (
    input wire clk,
    input wire rst,
    input wire raw_signal,
    output reg secure_signal,
    output reg irq_trigger
);
    reg [21:0] glitch_counter; 

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            glitch_counter <= 0;
            secure_signal  <= 0;
            irq_trigger    <= 0;
        end else begin
            if (raw_signal == 1'b1) begin
                if (glitch_counter < 22'd2000000) begin
                    glitch_counter <= glitch_counter + 1;
                    secure_signal  <= 1'b1;
                end else begin
                    secure_signal  <= 1'b0;
                    irq_trigger    <= 1'b1;
                end
            end else begin
                glitch_counter <= 0;
                secure_signal  <= 1'b0;
                irq_trigger    <= 1'b0;
            end
        end
    end
endmodule
