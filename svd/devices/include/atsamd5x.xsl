<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The NVMCTRL:PARAM:SEE field is specified as 1 bit wide, but several
    devices include much wider enumeration values that don't correspond to the
    actual values expected for this field. -->
  <xsl:template match="/device/peripherals/peripheral[name='NVMCTRL']/registers/register[name='PARAM']/fields/field[name='SEE']/enumeratedValues">
  </xsl:template>

  <!-- The SDHC0 peripheral on some devices contains reset values for certain
    fields that are wider than their corresponding storage. Replace the invalid
    values with the ones from the datasheet. -->
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R']/resetValue">
    0x00
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R_EMMC_MODE']/resetValue">
    0x00
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='SISR']/resetValue">
    0x00
  </xsl:template>

  <!-- The TCn::COUNTn::CTRLA::DMAOS fields are all missing. -->
  <xsl:template match="/device/peripherals/peripheral[name='TC0']/registers/cluster/register[name='CTRLA']/fields">
    <fields>
      <xsl:copy-of select="./field"/>
      <field>
        <name>DMAOS</name>
        <description>DMA One-Shot Trigger Mode</description>
        <bitOffset>15</bitOffset>
        <bitWidth>1</bitWidth>
      </field>
    </fields>
  </xsl:template>

  <!-- The GMAC::TIDM::ENIDn fields are all missing. -->
  <xsl:template match="/device/peripherals/peripheral[name='GMAC']/registers/register[name='TIDM[%s]']/fields">
    <fields>
      <xsl:copy-of select="./field"/>
      <field>
        <name>ENID</name>
        <description>Enable Copying of TID Matched Frames</description>
        <bitOffset>31</bitOffset>
        <bitWidth>1</bitWidth>
      </field>
    </fields>
  </xsl:template>

  <!-- The SERCOM USART TXPO enumerated values of the original SVD don't match the datasheet-->
  <xsl:template match = "/device/peripherals/peripheral[name='SERCOM0']/registers/cluster[name='USART_INT']/register[name='CTRLA']/fields/field[name='TXPO']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <enumeratedValue>
        <name>TXPO_0</name>
        <description>TxD on PAD0, XCK on PAD1</description>
        <value>0x0</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TXPO_2</name>
        <description>TxD on PAD0, RTS/TE on PAD2, CTS on PAD3</description>
        <value>0x2</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TXPO_3</name>
        <description>TxD on PAD0, XCK on PAD1, RTS/TE on PAD2</description>
        <value>0x3</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <xsl:template match = "/device/peripherals/peripheral[name='SERCOM0']/registers/cluster[name='USART_EXT']/register[name='CTRLA']/fields/field[name='TXPO']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <enumeratedValue>
        <name>TXPO_0</name>
        <description>TxD on PAD0, XCK on PAD1</description>
        <value>0x0</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TXPO_2</name>
        <description>TxD on PAD0, RTS/TE on PAD2, CTS on PAD3</description>
        <value>0x2</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TXPO_3</name>
        <description>TxD on PAD0, XCK on PAD1, RTS/TE on PAD2</description>
        <value>0x3</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- The DMAC trigger sources in the original SVD only have the 0=disabled
  enumeration value -->
  <xsl:template match="/device/peripherals/peripheral[name='DMAC']/registers/cluster/register[name='CHCTRLA']/fields/field[name='TRIGSRC']/enumeratedValues">
    <enumeratedValues>
      <xsl:copy-of select="./name"/>
      <xsl:copy-of select="./enumeratedValue"/>
      <enumeratedValue>
        <name>RTC_TIMESTAMP</name>
        <description>DMA RTC timestamp trigger</description>
        <value>0x01</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DSU_DCC0</name>
        <description>DMAC ID for DCC0 register</description>
        <value>0x02</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DSU_DCC1</name>
        <description>DMAC ID for DCC1 register</description>
        <value>0x03</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x04</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM0_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x05</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x06</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM1_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x07</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x08</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM2_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x09</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x0A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM3_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x0B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x0C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM4_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x0D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x0E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM5_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x0F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM6_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x10</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM6_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x11</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM7_RX</name>
        <description>Index of DMA RX trigger</description>
        <value>0x12</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>SERCOM7_TX</name>
        <description>Index of DMA TX trigger</description>
        <value>0x13</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>CAN0_DEBUG</name>
        <description>DMA CAN Debug Req</description>
        <value>0x14</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>CAN1_DEBUG</name>
        <description>DMA CAN Debug Req</description>
        <value>0x15</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_OVF</name>
        <description>DMA overflow/underflow/retrigger trigger</description>
        <value>0x16</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x17</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x18</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_2</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x19</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_3</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x1A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_4</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x1B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC0_MC_5</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x1C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_OVF</name>
        <description>DMA overflow/underflow/retrigger trigger</description>
        <value>0x1D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x1E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x1F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC_2</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x20</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC1_MC_3</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x21</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_OVF</name>
        <description>DMA overflow/underflow/retrigger trigger</description>
        <value>0x22</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x23</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x24</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC2_MC_2</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x25</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_OVF</name>
        <description>DMA overflow/underflow/retrigger trigger</description>
        <value>0x26</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x27</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC3_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x28</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC4_OVF</name>
        <description>DMA overflow/underflow/retrigger trigger</description>
        <value>0x29</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC4_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x2A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TCC4_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x2B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x2C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x2D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC0_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x2E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x2F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x30</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC1_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x31</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x32</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x33</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC2_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x34</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x35</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x36</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC3_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x37</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x38</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x39</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC4_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x3A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x3B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x3C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC5_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x3D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x3E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x3F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC6_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x40</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_OVF</name>
        <description>Indexes of DMA Overflow trigger</description>
        <value>0x41</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC_0</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x42</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>TC7_MC_1</name>
        <description>Indexes of DMA Match/Compare triggers</description>
        <value>0x43</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC0_RESRDY</name>
        <description>index of DMA RESRDY trigger</description>
        <value>0x44</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC0_SEQ</name>
        <description>Index of DMA SEQ trigger</description>
        <value>0x45</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC1_RESRDY</name>
        <description>Index of DMA RESRDY trigger</description>
        <value>0x46</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>ADC1_SEQ</name>
        <description>Index of DMA SEQ trigger</description>
        <value>0x47</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY_0</name>
        <description>DMA DAC Empty Req</description>
        <value>0x48</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_EMPTY_1</name>
        <description>DMA DAC Empty Req</description>
        <value>0x49</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_RESRDY_0</name>
        <description>DMA DAC Result Ready Req</description>
        <value>0x4A</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>DAC_RESRDY_1</name>
        <description>DMA DAC Result Ready Req</description>
        <value>0x4B</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_0</name>
        <description>Indexes of DMA RX triggers</description>
        <value>0x4C</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_RX_1</name>
        <description>Indexes of DMA RX triggers</description>
        <value>0x4D</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_0</name>
        <description>Indexes of DMA TX triggers</description>
        <value>0x4E</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>I2S_TX_1</name>
        <description>Indexes of DMA TX triggers</description>
        <value>0x4F</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>PCC_RX</name>
        <description>Indexes of PCC RX trigger</description>
        <value>0x50</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>AES_WR</name>
        <description>DMA DATA Write trigger</description>
        <value>0x51</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>AES_RD</name>
        <description>DMA DATA Read trigger</description>
        <value>0x52</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>QSPI_RX</name>
        <description>Indexes of QSPI RX trigger</description>
        <value>0x53</value>
      </enumeratedValue>
      <enumeratedValue>
        <name>QSPI_TX</name>
        <description>Indexes of QSPI TX trigger</description>
        <value>0x54</value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>
</xsl:stylesheet>
