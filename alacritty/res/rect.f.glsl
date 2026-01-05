#if defined(GLES2_RENDERER)
#define float_t mediump float
#define color_t mediump vec4
#define color3_t mediump vec3
#define coord_t mediump vec2
#define texture texture2D
#define FRAG_COLOR gl_FragColor

varying color_t color;

#else
#define float_t float
#define color_t vec4
#define color3_t vec3
#define coord_t vec2

out vec4 FragColor;
#define FRAG_COLOR FragColor

flat in color_t color;

#endif

uniform float_t cellWidth;
uniform float_t cellHeight;
uniform float_t screenWidth;
uniform float_t screenHeight;
uniform float_t paddingY;
uniform float_t paddingX;

uniform float_t underlinePosition;
uniform float_t underlineThickness;

uniform float_t undercurlPosition;

uniform sampler2D background;
uniform color3_t bgColor;

#define PI 3.1415926538

#if defined(DRAW_UNDERCURL)
color_t draw_undercurl(float_t x, float_t y) {
  // We use `undercurlPosition` as an amplitude, since it's half of the descent
  // value.
  //
  // The `x` represents the left bound of pixel we should add `1/2` to it, so
  // we compute the undercurl position for the center of the pixel.
  float_t undercurl = undercurlPosition / 2. * cos((x + 0.5) * 2.
                    * PI / cellWidth) + undercurlPosition - 1.;

  float_t undercurlTop = undercurl + max((underlineThickness - 1.), 0.) / 2.;
  float_t undercurlBottom = undercurl - max((underlineThickness - 1.), 0.) / 2.;

  // The distance to the curve boundary is always positive when it should
  // be used for AA. When both `y - undercurlTop` and `undercurlBottom - y`
  // expressions are negative, it means that the point is inside the curve
  // and we should just use alpha 1. To do so, we max one value with 0
  // so it'll use the alpha 1 in the end.
  float_t dst = max(y - undercurlTop, max(undercurlBottom - y, 0.));

  // Doing proper SDF is complicated for this shader, so just make AA
  // stronger by 1/x^2, which renders preserving underline thickness and
  // being bold enough.
  float_t alpha = 1. - dst * dst;

  // The result is an alpha mask on a rect, which leaves only curve opaque.
  return vec4(color.rgb, alpha);
}
#endif

#if defined(DRAW_DOTTED)
// When the dot size increases we can use AA to make spacing look even and the
// dots rounded.
color_t draw_dotted_aliased(float_t x, float_t y) {
  float_t dotNumber = floor(x / underlineThickness);

  float_t radius = underlineThickness / 2.;
  float_t centerY = underlinePosition - 1.;

  float_t leftCenter = (dotNumber - mod(dotNumber, 2.)) * underlineThickness + radius;
  float_t rightCenter = leftCenter + 2. * underlineThickness;

  float_t distanceLeft = sqrt(pow(x - leftCenter, 2.) + pow(y - centerY, 2.));
  float_t distanceRight = sqrt(pow(x - rightCenter, 2.) + pow(y - centerY, 2.));

  float_t alpha = max(1. - (min(distanceLeft, distanceRight) - radius), 0.);
  return vec4(color.rgb, alpha);
}

/// Draw dotted line when dot is just a single pixel.
color_t draw_dotted(float_t x, float_t y) {
  float_t cellEven = 0.;

  // Since the size of the dot and its gap combined is 2px we should ensure that
  // spacing will be even. If the cellWidth is even it'll work since we start
  // with dot and end with gap. However if cellWidth is odd, the cell will start
  // and end with a dot, creating a dash. To resolve this issue, we invert the
  // pattern every two cells.
  if (int(mod(cellWidth, 2.)) != 0) {
    cellEven = mod((gl_FragCoord.x - paddingX) / cellWidth, 2.);
  }

  // Since we use the entire descent area for dotted underlines, we limit its
  // height to a single pixel so we don't draw bars instead of dots.
  float_t alpha = 1. - abs(floor(underlinePosition) - y);
  if (int(mod(x, 2.)) != int(cellEven)) {
    alpha = 0.;
  }

  return vec4(color.rgb, alpha);
}
#endif

#if defined(DRAW_DASHED)
color_t draw_dashed(float_t x) {
  // Since dashes of adjacent cells connect with each other our dash length is
  // half of the desired total length.
  float_t halfDashLen = floor(cellWidth / 4. + 0.5);

  float_t alpha = 1.;

  // Check if `x` coordinate is where we should draw gap.
  if (x > halfDashLen - 1. && x < cellWidth - halfDashLen) {
    alpha = 0.;
  }

  return vec4(color.rgb, alpha);
}
#endif

void main() {
  float_t x = floor(mod(gl_FragCoord.x - paddingX, cellWidth));
  float_t y = floor(mod(gl_FragCoord.y - paddingY, cellHeight));

#if defined(DRAW_UNDERCURL)
  FRAG_COLOR = draw_undercurl(x, y);
#elif defined(DRAW_DOTTED)
  if (underlineThickness < 2.) {
    FRAG_COLOR = draw_dotted(x, y);
  } else {
    FRAG_COLOR = draw_dotted_aliased(x, y);
  }
#elif defined(DRAW_DASHED)
  FRAG_COLOR = draw_dashed(x);
#elif defined(DRAW_INVERTING)
  coord_t coord = vec2(gl_FragCoord.x / screenWidth, gl_FragCoord.y / screenHeight);
  color_t colr = texture(background, coord);
  float_t factor = length(bgColor / 255.0 - colr.rgb);
  // If colr is too close to cursor color, either brighten it or dim it to be
  // distinct enough
  color3_t col  = colr.rgb;
  // Colors must be separated by _at least_ 0.5 on the "value" scale
  float_t V_our = max(color.r, max(color.g, color.b));
  float_t V_tgt = max(col.r, max(col.g, col.b));
  // V_tgt == 0 is absolute black and is impossible to "scale" proportionally
  // so we just leave it be. This works fine as far as I can tell
  if (V_tgt != 0.0 && abs(V_our - V_tgt) < 0.5) {
      if (V_our > 0.5) {
          // Bring V_tgt to V_our - 0.5f
          col *= (V_our - 0.5) / V_tgt;
      } else {
          // Bring V_tgt to V_our + 0.5f
          col *= (V_our + 0.5) / V_tgt;
      }
  }
  // Final cursor color
  color3_t curc = mix(color.rgb, col, factor);
  // Blend with opacity
  FRAG_COLOR = mix(colr, vec4(curc, color.a), color.a);
#else
  coord_t coord = vec2(gl_FragCoord.x / screenWidth, gl_FragCoord.y / screenHeight);
  FRAG_COLOR = texture(background, coord) * (1.0 - color.a) + color * color.a;
#endif
}
