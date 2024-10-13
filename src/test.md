高斯积分，又称高斯函数积分，是数学中一个重要的积分，通常指的是以下形式的积分：

\[ \int_{-\infty}^{\infty} e^{-x^2} \, dx \]

这个积分的结果是 \(\sqrt{\pi}\)。下面是计算这个积分的一种常用方法：

### 方法一：平方化法

1. **定义积分**：
   \[ I = \int_{-\infty}^{\infty} e^{-x^2} \, dx \]

2. **平方化**：
   \[ I^2 = \left( \int_{-\infty}^{\infty} e^{-x^2} \, dx \right)^2 \]

3. **转换为二重积分**：
   \[ I^2 = \int_{-\infty}^{\infty} e^{-x^2} \, dx \cdot \int_{-\infty}^{\infty} e^{-y^2} \, dy \]
   \[ I^2 = \int_{-\infty}^{\infty} \int_{-\infty}^{\infty} e^{-(x^2 + y^2)} \, dx \, dy \]

4. **极坐标变换**：
   令 \( x = r \cos \theta \)，\( y = r \sin \theta \)，则 \( dx \, dy = r \, dr \, d\theta \)，积分区域变为 \( (0, \infty) \times (0, 2\pi) \)。
   \[ I^2 = \int_{0}^{2\pi} \int_{0}^{\infty} e^{-r^2} \cdot r \, dr \, d\theta \]

5. **分离积分**：
   \[ I^2 = \left( \int_{0}^{2\pi} d\theta \right) \left( \int_{0}^{\infty} e^{-r^2} \cdot r \, dr \right) \]
   \[ I^2 = 2\pi \int_{0}^{\infty} e^{-r^2} \cdot r \, dr \]

6. **计算内积分**：
   令 \( u = r^2 \)，则 \( du = 2r \, dr \)，
   \[ \int_{0}^{\infty} e^{-r^2} \cdot r \, dr = \frac{1}{2} \int_{0}^{\infty} e^{-u} \, du = \frac{1}{2} \left[ -e^{-u} \right]_{0}^{\infty} = \frac{1}{2} \]

7. **代入并求解**：
   \[ I^2 = 2\pi \cdot \frac{1}{2} = \pi \]
   \[ I = \sqrt{\pi} \]

### 方法二：变量替换法

1. **定义积分**：
   \[ I = \int_{-\infty}^{\infty} e^{-x^2} \, dx \]

2. **变量替换**：
   令 \( t = \sqrt{2} x \)，则 \( dt = \sqrt{2} \, dx \)，
   \[ I = \int_{-\infty}^{\infty} e^{-x^2} \, dx = \frac{1}{\sqrt{2}} \int_{-\infty}^{\infty} e^{-\frac{t^2}{2}} \, dt \]

3. **利用标准正态分布的积分结果**：
   标准正态分布的积分结果是 \(\sqrt{2\pi}\)，
   \[ \int_{-\infty}^{\infty} e^{-\frac{t^2}{2}} \, dt = \sqrt{2\pi} \]

4. **代入并求解**：
   \[ I = \frac{1}{\sqrt{2}} \cdot \sqrt{2\pi} = \sqrt{\pi} \]

### 结论

无论使用哪种方法，高斯积分的结果都是 \(\sqrt{\pi}\)。

