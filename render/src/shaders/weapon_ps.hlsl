// Studio lighting for the owned preview; linear shading, filmic output.
cbuffer Scene : register(b0) {
 float4x4 gViewProj; float4x4 gWorld;
 float4 gLightDir; float4 gCamPos; float4 gTint;
};
Texture2D gAlbedo : register(t0);
SamplerState gSampler : register(s0);
struct PS_IN { float4 pos:SV_POSITION; float3 normal:NORMAL; float2 uv:TEXCOORD0; float3 world:TEXCOORD1; };
float4 main(PS_IN i):SV_Target {
 float4 tex=gAlbedo.Sample(gSampler,i.uv);
 float3 base=pow(max(tex.rgb,0.0),2.2);
 float3 n=normalize(i.normal),v=normalize(gCamPos.xyz-i.world);
 float3 key=normalize(float3(-.45,.65,.8));
 float3 fill=normalize(float3(.7,.15,.6));
 float3 rim=normalize(float3(.3,.6,-.8));
 float diffuse=.32+.95*saturate(dot(n,key))+.28*saturate(dot(n,fill));
 float spec=pow(saturate(dot(n,normalize(key+v))),64.0)*.04;
 float edge=pow(1.0-saturate(dot(n,v)),3.0)*saturate(dot(n,rim));
 float3 color=base*diffuse+spec+edge*float3(.09,.14,.22);
 color=color/(1.0+color*.3);
 // Opaque texture alpha is material data. Native BLEND lenses/decals use alpha.
 float opacity=lerp(tex.a,1.0,gTint.w);
 return float4(pow(saturate(color),1.0/2.2),opacity);
}
