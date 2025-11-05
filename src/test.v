module lite_v1
    (
        input            clk,
        input            rst,
        input      [1:0] opcode,
        input      [7:0] d1,
        input      [9:0] d2,
        output reg [7:0] res
    );

    

    always @(posedge clk) begin
        if (rst == 1'b1) begin
            res <= 8'b0;
        end else begin
            case (opcode)
                1'b0:    res <= d1;
                1'b1:    res <= d2;
                default: res <= 8'b0;
            endcase
        end
    end

endmodule
