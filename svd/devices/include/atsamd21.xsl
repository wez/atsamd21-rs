<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="./ptc.xsl"/>
  <!-- The DMAC trigger sources in the original SVD only have the 0=disabled
  enumeration value -->
  <xsl:template match="/device/peripherals/peripheral[name='DMAC']/registers/register[name='CHCTRLB']/fields/field[name='TRIGSRC']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <xsl:copy-of select="./enumeratedValue"/>
      <enumeratedValue>
        <name>SERCOM0_RX</name>
        <description>SERCOM0 RX Trigger</description>
        <value>0x01</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_TX</name>
        <description>SERCOM0 TX Trigger</description>
        <value>0x02</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_RX</name>
        <description>SERCOM1 RX Trigger</description>
        <value>0x03</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_TX</name>
        <description>SERCOM1 TX Trigger</description>
        <value>0x04</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_RX</name>
        <description>SERCOM2 RX Trigger</description>
        <value>0x05</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_TX</name>
        <description>SERCOM2 TX Trigger</description>
        <value>0x06</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_RX</name>
        <description>SERCOM3 RX Trigger</description>
        <value>0x07</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_TX</name>
        <description>SERCOM3 TX Trigger</description>
        <value>0x08</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_RX</name>
        <description>SERCOM4 RX Trigger</description>
        <value>0x09</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_TX</name>
        <description>SERCOM4 TX Trigger</description>
        <value>0x0A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_RX</name>
        <description>SERCOM5 RX Trigger</description>
        <value>0x0B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_TX</name>
        <description>SERCOM5 TX Trigger</description>
        <value>0x0C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_OVF</name>
        <description>TCC0 Overflow Trigger</description>
        <value>0x0D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC0</name>
        <description>TCC0 Match/Compare 0 Trigger</description>
        <value>0x0E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC1</name>
        <description>TCC0 Match/Compare 1 Trigger</description>
        <value>0x0F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC2</name>
        <description>TCC0 Match/Compare 2 Trigger</description>
        <value>0x10</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC3</name>
        <description>TCC0 Match/Compare 3 Trigger</description>
        <value>0x11</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_OVF</name>
        <description>TCC1 Overflow Trigger</description>
        <value>0x12</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC0</name>
        <description>TCC1 Match/Compare 0 Trigger</description>
        <value>0x13</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC1</name>
        <description>TCC1 Match/Compare 1 Trigger</description>
        <value>0x14</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_OVF</name>
        <description>TCC2 Overflow Trigger</description>
        <value>0x15</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC0</name>
        <description>TCC2 Match/Compare 0 Trigger</description>
        <value>0x16</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC1</name>
        <description>TCC2 Match/Compare 1 Trigger</description>
        <value>0x17</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_OVF</name>
        <description>TC3 Overflow Trigger</description>
        <value>0x18</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC0</name>
        <description>TC3 Match/Compare 0 Trigger</description>
        <value>0x19</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC1</name>
        <description>TC3 Match/Compare 1 Trigger</description>
        <value>0x1A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_OVF</name>
        <description>TC4 Overflow Trigger</description>
        <value>0x1B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC0</name>
        <description>TC4 Match/Compare 0 Trigger</description>
        <value>0x1C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC1</name>
        <description>TC4 Match/Compare 1 Trigger</description>
        <value>0x1D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_OVF</name>
        <description>TC5 Overflow Trigger</description>
        <value>0x1E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC0</name>
        <description>TC5 Match/Compare 0 Trigger</description>
        <value>0x1F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC1</name>
        <description>TC5 Match/Compare 1 Trigger</description>
        <value>0x20</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_OVF</name>
        <description>TC6 Overflow Trigger</description>
        <value>0x21</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC0</name>
        <description>TC6 Match/Compare 0 Trigger</description>
        <value>0x22</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC1</name>
        <description>TC6 Match/Compare 1 Trigger</description>
        <value>0x23</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_OVF</name>
        <description>TC7 Overflow Trigger</description>
        <value>0x24</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC0</name>
        <description>TC7 Match/Compare 0 Trigger</description>
        <value>0x25</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC1</name>
        <description>TC7 Match/Compare 1 Trigger</description>
        <value>0x26</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC_RESRDY</name>
        <description>ADC Result Ready Trigger</description>
        <value>0x27</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY</name>
        <description>DAC Empty Trigger</description>
        <value>0x28</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_0</name>
        <description>I2S RX 0 Trigger</description>
        <value>0x29</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_1</name>
        <description>I2S RX 1 Trigger</description>
        <value>0x2A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_0</name>
        <description>I2S TX 0 Trigger</description>
        <value>0x2B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_1</name>
        <description>I2S TX 1 Trigger</description>
        <value>0x2C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_OVF</name>
        <description>TCC3 Overflow Trigger</description>
        <value>0x2D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC0</name>
        <description>TCC3 Match/Compare 0 Trigger</description>
        <value>0x2E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC1</name>
        <description>TCC3 Match/Compare 1 Trigger</description>
        <value>0x2F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC2</name>
        <description>Match/Compare 2 Trigger</description>
        <value>0x30</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC3</name>
        <description>Match/Compare 3 Trigger</description>
        <value>0x31</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- Some register bits are erroneously write-only in the
  original SVD, remove the access field that marks it as such -->
  <xsl:template match="/device/peripherals/peripheral[name='PORT']/registers/register[name='PINCFG0_%s']/fields/field[name='DRVSTR']/access" />
  <xsl:template match="/device/peripherals/peripheral[name='RTC']/registers/cluster/register[name='CTRL']/fields/field[name='SWRST']/access" />

  <!-- Add enumeratedValue for PTC in GCLK CLKCTRL register's ID field -->
  <xsl:template match="/device/peripherals/peripheral[name='GCLK']/registers/register[name='CLKCTRL']/fields/field[name='ID']/enumeratedValues">
    <xsl:call-template name="PTC_GCLK_ID">
      <xsl:with-param name="PTC_GCLK_ID_NAME">PTC</xsl:with-param>
      <xsl:with-param name="PTC_GCLK_ID_DESC">PTCReserved</xsl:with-param>
      <xsl:with-param name="PTC_GCLK_ID_VALUE">0x22</xsl:with-param>
    </xsl:call-template>
  </xsl:template>

  <!-- Add PTC peripheral -->
  <xsl:template match="/device/peripherals">
    <xsl:call-template name="PTC_PERIPHERAL">
      <xsl:with-param name="PTC_PERIPHERAL_BASEADDRESS">0x42004C00</xsl:with-param>
      <xsl:with-param name="PTC_PERIPHERAL_INTERRUPT_VALUE">26</xsl:with-param>
    </xsl:call-template>
  </xsl:template>
</xsl:stylesheet>
