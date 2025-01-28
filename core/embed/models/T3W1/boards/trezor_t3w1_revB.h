#ifndef TREZOR_T3W1_REVA_H_
#define TREZOR_T3W1_REVA_H_

#define VDD_1V8 1
#define USE_SMPS 1

#define BTN_POWER_PIN GPIO_PIN_5
#define BTN_POWER_PORT GPIOE
#define BTN_POWER_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define BTN_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOE
#define BTN_EXTI_INTERRUPT_LINE EXTI_LINE_5
#define BTN_EXTI_INTERRUPT_PIN GPIO_PIN_5
#define BTN_EXTI_INTERRUPT_NUM EXTI5_IRQn
#define BTN_EXTI_INTERRUPT_HANDLER EXTI5_IRQHandler

#define DISPLAY_COLOR_MODE DMA2D_OUTPUT_ARGB8888
#define DISPLAY_PANEL_LX250A2401A
#define DISPLAY_GFXMMU 1
#define DISPLAY_RESET_PIN GPIO_PIN_2
#define DISPLAY_RESET_PORT GPIOE
#define DISPLAY_RESET_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define DISPLAY_PWREN_PIN GPIO_PIN_1
#define DISPLAY_PWREN_PORT GPIOG
#define DISPLAY_PWREN_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE

#define BACKLIGHT_PWM_FREQ 200
#define BACKLIGHT_PWM_TIM TIM17
#define BACKLIGHT_PWM_TIM_CLK_EN __HAL_RCC_TIM17_CLK_ENABLE
#define BACKLIGHT_PWM_TIM_CLK_DIS __HAL_RCC_TIM17_CLK_DISABLE
#define BACKLIGHT_PWM_TIM_FORCE_RESET __HAL_RCC_TIM17_FORCE_RESET
#define BACKLIGHT_PWM_TIM_RELEASE_RESET __HAL_RCC_TIM17_RELEASE_RESET
#define BACKLIGHT_PWM_TIM_AF GPIO_AF14_TIM17
#define BACKLIGHT_PWM_TIM_OCMODE TIM_OCMODE_PWM1
#define BACKLIGHT_PWM_TIM_CHANNEL TIM_CHANNEL_1
#define BACKLIGHT_PWM_TIM_CCR CCR1
#define BACKLIGHT_PWM_PIN GPIO_PIN_9
#define BACKLIGHT_PWM_PORT GPIOB
#define BACKLIGHT_PWM_PORT_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE

#define NPM1300_I2C_INSTANCE 0

#define STWLC38_I2C_INSTANCE 1

#define I2C_COUNT 4

#define I2C_INSTANCE_0 I2C1
#define I2C_INSTANCE_0_CLK_EN __HAL_RCC_I2C1_CLK_ENABLE
#define I2C_INSTANCE_0_CLK_DIS __HAL_RCC_I2C1_CLK_DISABLE
#define I2C_INSTANCE_0_PIN_AF GPIO_AF4_I2C1
#define I2C_INSTANCE_0_SDA_PORT GPIOG
#define I2C_INSTANCE_0_SDA_PIN GPIO_PIN_13
#define I2C_INSTANCE_0_SDA_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE
#define I2C_INSTANCE_0_SCL_PORT GPIOG
#define I2C_INSTANCE_0_SCL_PIN GPIO_PIN_14
#define I2C_INSTANCE_0_SCL_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE
#define I2C_INSTANCE_0_RESET_REG &RCC->APB1RSTR1
#define I2C_INSTANCE_0_RESET_BIT RCC_APB1RSTR1_I2C1RST
#define I2C_INSTANCE_0_EV_IRQHandler I2C1_EV_IRQHandler
#define I2C_INSTANCE_0_ER_IRQHandler I2C1_ER_IRQHandler
#define I2C_INSTANCE_0_EV_IRQn I2C1_EV_IRQn
#define I2C_INSTANCE_0_ER_IRQn I2C1_ER_IRQn
#define I2C_INSTANCE_0_GUARD_TIME 0

#define I2C_INSTANCE_1 I2C2
#define I2C_INSTANCE_1_CLK_EN __HAL_RCC_I2C2_CLK_ENABLE
#define I2C_INSTANCE_1_CLK_DIS __HAL_RCC_I2C2_CLK_DISABLE
#define I2C_INSTANCE_1_PIN_AF GPIO_AF4_I2C2
#define I2C_INSTANCE_1_SDA_PORT GPIOF
#define I2C_INSTANCE_1_SDA_PIN GPIO_PIN_0
#define I2C_INSTANCE_1_SDA_CLK_EN __HAL_RCC_GPIOF_CLK_ENABLE
#define I2C_INSTANCE_1_SCL_PORT GPIOF
#define I2C_INSTANCE_1_SCL_PIN GPIO_PIN_1
#define I2C_INSTANCE_1_SCL_CLK_EN __HAL_RCC_GPIOF_CLK_ENABLE
#define I2C_INSTANCE_1_RESET_REG &RCC->APB1RSTR1
#define I2C_INSTANCE_1_RESET_BIT RCC_APB1RSTR1_I2C2RST
#define I2C_INSTANCE_1_EV_IRQHandler I2C2_EV_IRQHandler
#define I2C_INSTANCE_1_ER_IRQHandler I2C2_ER_IRQHandler
#define I2C_INSTANCE_1_EV_IRQn I2C2_EV_IRQn
#define I2C_INSTANCE_1_ER_IRQn I2C2_ER_IRQn
#define I2C_INSTANCE_1_GUARD_TIME 0

#define I2C_INSTANCE_2 I2C3
#define I2C_INSTANCE_2_CLK_EN __HAL_RCC_I2C3_CLK_ENABLE
#define I2C_INSTANCE_2_CLK_DIS __HAL_RCC_I2C3_CLK_DISABLE
#define I2C_INSTANCE_2_PIN_AF GPIO_AF4_I2C3
#define I2C_INSTANCE_2_SDA_PORT GPIOC
#define I2C_INSTANCE_2_SDA_PIN GPIO_PIN_1
#define I2C_INSTANCE_2_SDA_CLK_EN __HAL_RCC_GPIOC_CLK_ENABLE
#define I2C_INSTANCE_2_SCL_PORT GPIOC
#define I2C_INSTANCE_2_SCL_PIN GPIO_PIN_0
#define I2C_INSTANCE_2_SCL_CLK_EN __HAL_RCC_GPIOC_CLK_ENABLE
#define I2C_INSTANCE_2_RESET_REG &RCC->APB3RSTR
#define I2C_INSTANCE_2_RESET_BIT RCC_APB3RSTR_I2C3RST
#define I2C_INSTANCE_2_EV_IRQHandler I2C3_EV_IRQHandler
#define I2C_INSTANCE_2_ER_IRQHandler I2C3_ER_IRQHandler
#define I2C_INSTANCE_2_EV_IRQn I2C3_EV_IRQn
#define I2C_INSTANCE_2_ER_IRQn I2C3_ER_IRQn
#define I2C_INSTANCE_2_GUARD_TIME 0

#define I2C_INSTANCE_3 I2C4
#define I2C_INSTANCE_3_CLK_EN __HAL_RCC_I2C4_CLK_ENABLE
#define I2C_INSTANCE_3_CLK_DIS __HAL_RCC_I2C4_CLK_DISABLE
#define I2C_INSTANCE_3_PIN_AF GPIO_AF4_I2C4
#define I2C_INSTANCE_3_SDA_PORT GPIOD
#define I2C_INSTANCE_3_SDA_PIN GPIO_PIN_13
#define I2C_INSTANCE_3_SDA_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_3_SCL_PORT GPIOD
#define I2C_INSTANCE_3_SCL_PIN GPIO_PIN_12
#define I2C_INSTANCE_3_SCL_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_3_RESET_REG &RCC->APB1RSTR2
#define I2C_INSTANCE_3_RESET_BIT RCC_APB1RSTR2_I2C4RST
#define I2C_INSTANCE_3_EV_IRQHandler I2C4_EV_IRQHandler
#define I2C_INSTANCE_3_ER_IRQHandler I2C4_ER_IRQHandler
#define I2C_INSTANCE_3_EV_IRQn I2C4_EV_IRQn
#define I2C_INSTANCE_3_ER_IRQn I2C4_ER_IRQn
#define I2C_INSTANCE_3_GUARD_TIME 50

#define TOUCH_SENSITIVITY 0x40
#define TOUCH_I2C_INSTANCE 2
#define TOUCH_INT_PORT GPIOC
#define TOUCH_INT_PIN GPIO_PIN_3

#define DRV2625_I2C_INSTANCE 2
#define HAPTIC_ACTUATOR "actuators/ld0625bc.h"
#define DRV2625_TRIG_PIN GPIO_PIN_2
#define DRV2625_TRIG_PORT GPIOA
#define DRV2625_TRIG_CLK_ENA __HAL_RCC_GPIOA_CLK_ENABLE
#define DRV2625_TRIG_AF GPIO_AF14_TIM15
#define DRV2625_TRIG_TIM TIM15
#define DRV2625_TRIG_TIM_CLK_ENA __HAL_RCC_TIM15_CLK_ENABLE
#define DRV2625_TRIG_TIM_CLK_DIS __HAL_RCC_TIM15_CLK_DISABLE
#define DRV2625_TRIG_TIM_FORCE_RESET __HAL_RCC_TIM15_FORCE_RESET
#define DRV2625_TRIG_TIM_RELEASE_RESET __HAL_RCC_TIM15_RELEASE_RESET
#define DRV2625_RESET_PIN GPIO_PIN_3
#define DRV2625_RESET_PORT GPIOA
#define DRV2625_RESET_CLK_ENA __HAL_RCC_GPIOA_CLK_ENABLE

#define OPTIGA_I2C_INSTANCE 3
#define OPTIGA_RST_PORT GPIOD
#define OPTIGA_RST_PIN GPIO_PIN_10
#define OPTIGA_RST_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define OPTIGA_PWR_PORT GPIOD
#define OPTIGA_PWR_PIN GPIO_PIN_14
#define OPTIGA_PWR_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE

#define SBU_1_PIN GPIO_PIN_8
#define SBU_1_PORT GPIOC
#define SBU_1_CLK_ENA __HAL_RCC_GPIOC_CLK_ENABLE
#define SBU_2_PIN GPIO_PIN_9
#define SBU_2_PORT GPIOC
#define SBU_2_CLK_ENA __HAL_RCC_GPIOC_CLK_ENABLE

#define NRF_IN_GPIO0_PIN GPIO_PIN_7
#define NRF_IN_GPIO0_PORT GPIOE
#define NRF_IN_GPIO0_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_IN_FW_RUNNING_PIN GPIO_PIN_13
#define NRF_IN_FW_RUNNING_PORT GPIOE
#define NRF_IN_FW_RUNNING_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_OUT_RESET_PIN GPIO_PIN_0
#define NRF_OUT_RESET_PORT GPIOG
#define NRF_OUT_RESET_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE
#define NRF_OUT_STAY_IN_BLD_PIN GPIO_PIN_15
#define NRF_OUT_STAY_IN_BLD_PORT GPIOE
#define NRF_OUT_STAY_IN_BLD_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_OUT_FW_RUNNING_PIN GPIO_PIN_11
#define NRF_OUT_FW_RUNNING_PORT GPIOE
#define NRF_OUT_FW_RUNNING_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE

#endif  // TREZOR_T3W1_REVA_H_
