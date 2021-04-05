<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="include/common.xsl"/>
  <xsl:include href="include/ptc.xsl"/>

  <xsl:template match="/device/peripherals/peripheral[name='USB']/registers/cluster[name='DEVICE']/register[name='FSMSTATUS']/fields/field[name='FSMSTATE']/bitWidth">
    <bitWidth>7</bitWidth>
  </xsl:template>

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
        <name>TCC0_OVF</name>
        <description>TCC0 Overflow Trigger</description>
        <value>0x07</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC0</name>
        <description>TCC0 Match/Compare 0 Trigger</description>
        <value>0x08</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC1</name>
        <description>TCC0 Match/Compare 1 Trigger</description>
        <value>0x09</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC2</name>
        <description>TCC0 Match/Compare 2 Trigger</description>
        <value>0x0A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC3</name>
        <description>TCC0 Match/Compare 3 Trigger</description>
        <value>0x0B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_OVF</name>
        <description>TC1 Overflow Trigger</description>
        <value>0x0C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC0</name>
        <description>TC1 Match/Compare 0 Trigger</description>
        <value>0x0D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC1</name>
        <description>TC1 Match/Compare 1 Trigger</description>
        <value>0x0E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_OVF</name>
        <description>TC2 Overflow Trigger</description>
        <value>0x0F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC0</name>
        <description>TC2 Match/Compare 0 Trigger</description>
        <value>0x10</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC1</name>
        <description>TC2 Match/Compare 1 Trigger</description>
        <value>0x11</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC_RESRDY</name>
        <description>ADC Result Ready Trigger</description>
        <value>0x12</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY</name>
        <description>DAC Empty Trigger</description>
        <value>0x13</value>
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
      <xsl:with-param name="PTC_GCLK_ID_DESC">PTC</xsl:with-param>
      <xsl:with-param name="PTC_GCLK_ID_VALUE">0x17</xsl:with-param>
    </xsl:call-template>
  </xsl:template>

  <!-- Add PTC peripheral -->
  <xsl:template match="/device/peripherals">
    <xsl:call-template name="PTC_PERIPHERAL">
      <xsl:with-param name="PTC_PERIPHERAL_BASEADDRESS">0x42004C00</xsl:with-param>
      <xsl:with-param name="PTC_PERIPHERAL_INTERRUPT_VALUE">18</xsl:with-param>
    </xsl:call-template>
  </xsl:template>
</xsl:stylesheet>
