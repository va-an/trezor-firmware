#ifndef STM32U5A9J_DK_H_
#define STM32U5A9J_DK_H_

#define VDD_1V8 1

#define DISPLAY_COLOR_MODE DMA2D_OUTPUT_ARGB8888
#define DISPLAY_PANEL_STM32U5A9J_DK
// #define DISPLAY_GFXMMU 1
#define DISPLAY_RESET_PIN GPIO_PIN_5
#define DISPLAY_RESET_PORT GPIOD
#define DISPLAY_RESET_CLK_ENA __HAL_RCC_GPIOD_CLK_ENABLE

#define BACKLIGHT_PIN_PIN GPIO_PIN_6
#define BACKLIGHT_PIN_PORT GPIOI
#define BACKLIGHT_PIN_CLK_ENABLE() __HAL_RCC_GPIOI_CLK_ENABLE()

#define I2C_COUNT 1
#define I2C_INSTANCE_0 I2C5
#define I2C_INSTANCE_0_CLK_EN __HAL_RCC_I2C5_CLK_ENABLE
#define I2C_INSTANCE_0_CLK_DIS __HAL_RCC_I2C5_CLK_DISABLE
#define I2C_INSTANCE_0_PIN_AF GPIO_AF2_I2C5
#define I2C_INSTANCE_0_SDA_PORT GPIOH
#define I2C_INSTANCE_0_SDA_PIN GPIO_PIN_4
#define I2C_INSTANCE_0_SDA_CLK_EN __HAL_RCC_GPIOH_CLK_ENABLE
#define I2C_INSTANCE_0_SCL_PORT GPIOH
#define I2C_INSTANCE_0_SCL_PIN GPIO_PIN_5
#define I2C_INSTANCE_0_SCL_CLK_EN __HAL_RCC_GPIOH_CLK_ENABLE
#define I2C_INSTANCE_0_RESET_REG &RCC->APB1RSTR2
#define I2C_INSTANCE_0_RESET_BIT RCC_APB1RSTR2_I2C5RST
#define I2C_INSTANCE_0_EV_IRQHandler I2C5_EV_IRQHandler
#define I2C_INSTANCE_0_ER_IRQHandler I2C5_ER_IRQHandler
#define I2C_INSTANCE_0_EV_IRQn I2C5_EV_IRQn
#define I2C_INSTANCE_0_ER_IRQn I2C5_ER_IRQn
#define I2C_INSTANCE_0_GUARD_TIME 0

#define TOUCH_I2C_INSTANCE 0

#endif  // STM32U5A9J_DK_H_
