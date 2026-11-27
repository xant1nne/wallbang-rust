struct VS_OUT {
    float4 pos : SV_POSITION;
    float2 uv  : TEXCOORD0;
};

VS_OUT main(uint id : SV_VertexID) {
    VS_OUT o;
    float2 uv = float2((id << 1) & 2, id & 2);
    o.uv = uv;
    o.pos = float4(uv * float2(2.0f, -2.0f) + float2(-1.0f, 1.0f), 0.0f, 1.0f);
    return o;
}
