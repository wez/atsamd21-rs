<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- Add PTCReserved to the Generic Clock Selection ID enum on the GCLK peripheral -->
  <xsl:template name="PTC_GCLK_ID">
    <xsl:param name="PTC_GCLK_ID_NAME" required="yes" as="xs:string"/>
    <xsl:param name="PTC_GCLK_ID_DESC" required="yes" as="xs:string"/>
    <xsl:param name="PTC_GCLK_ID_VALUE" required="yes" as="xs:integer"/>
    <enumeratedValues>
      <xsl:for-each select="./enumeratedValue">
        <xsl:copy-of select="."/>
      </xsl:for-each>
      <enumeratedValue>
        <name><xsl:value-of select="$PTC_GCLK_ID_NAME"/></name>
        <description><xsl:value-of select="$PTC_GCLK_ID_DESC"/></description>
        <value><xsl:value-of select="$PTC_GCLK_ID_VALUE"/></value>
      </enumeratedValue>
    </enumeratedValues>
  </xsl:template>

  <!-- Add PTC peripheral. This is a best guess based off Adafruit's FreeTouch repo. -->
  <xsl:template name="PTC_PERIPHERAL">
    <xsl:param name="PTC_PERIPHERAL_BASEADDRESS" required="yes" as="xs:integer"/>
    <xsl:param name="PTC_PERIPHERAL_INTERRUPT_VALUE" required="yes" as="xs:integer"/>
    <peripherals>
      <xsl:for-each select="./peripheral">
        <xsl:apply-templates select="."/>
      </xsl:for-each>
      <peripheral>
        <name>PTC</name>
        <version>1.0.0</version>
        <description>Peripheral Touch Controller</description>
        <groupName>PTC</groupName>
        <prependToName>PTC_</prependToName>
        <baseAddress><xsl:value-of select="$PTC_PERIPHERAL_BASEADDRESS"/></baseAddress>
        <addressBlock>
          <offset>0</offset>
          <size>0x28</size>
          <usage>registers</usage>
        </addressBlock>
        <interrupt>
          <name>PTC</name>
          <value><xsl:value-of select="$PTC_PERIPHERAL_INTERRUPT_VALUE"/></value>
        </interrupt>
        <registers>
          <register>
            <name>CTRLA</name>
            <description>Control A</description>
            <addressOffset>0x00</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SWRST</name>
                <description>Software Reset</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>ENABLE</name>
                <description>Enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>RUNSTDBY</name>
                <description>Run in Standby</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>CTRLB</name>
            <description>Control B</description>
            <addressOffset>0x01</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SYNCFLAG</name>
                <description>Synchronisation flag</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <!-- Maybe this is like REFCTRL in ADC? -->
            <name>UNK4C04</name>
            <description>Unknown Register 0x42004C04</description>
            <addressOffset>0x04</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>CTRLC</name>
            <description>Control C</description>
            <addressOffset>0x05</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>INIT</name>
                <description>Initialize</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTENCLR</name>
            <description>Interrupt Enable Clear</description>
            <addressOffset>0x08</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt on end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTENSET</name>
            <description>Interrupt Enable Set</description>
            <addressOffset>0x09</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt on end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTFLAG</name>
            <description>Interrupt Flag Status and Clear</description>
            <addressOffset>0x0A</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>EOC</name>
                <description>Interrupt end of comparison</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>WCO</name>
                <description>Watch crystal oscillator interrupt</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>FREQCTRL</name>
            <description>Frequency Control</description>
            <addressOffset>0x0C</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>SAMPLEDELAY</name>
                <description>Sample delay</description>
                <bitOffset>0</bitOffset>
                <bitWidth>4</bitWidth>
                <enumeratedValues>
                  <name>FREQHOP</name>
                  <enumeratedValue>
                    <name>FREQHOP1</name>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP2</name>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP3</name>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP4</name>
                    <value>0x3</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP5</name>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP6</name>
                    <value>0x5</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP7</name>
                    <value>0x6</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP8</name>
                    <value>0x7</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP9</name>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP10</name>
                    <value>0x9</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP11</name>
                    <value>0xA</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP12</name>
                    <value>0xB</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP13</name>
                    <value>0xC</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP14</name>
                    <value>0xD</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP15</name>
                    <value>0xE</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>FREQHOP16</name>
                    <value>0xF</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
              <field>
                <name>FREQSPREADEN</name>
                <description>Enable frequency spread</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>CONVCTRL</name>
            <description>Conversion control</description>
            <addressOffset>0x0D</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>ADCACCUM</name>
                <description>ADC Accumulator</description>
                <bitOffset>0</bitOffset>
                <bitWidth>3</bitWidth>
                <enumeratedValues>
                  <name>OVERSAMPLECOUNT</name>
                  <enumeratedValue>
                    <name>OVERSAMPLE1</name>
                    <description>1 sample per report</description>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE2</name>
                    <description>2 samples per report</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE4</name>
                    <description>4 samples per report</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE8</name>
                    <description>8 samples per report</description>
                    <value>0x3</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE16</name>
                    <description>16 samples per report</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE32</name>
                    <description>32 samples per report</description>
                    <value>0x5</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>OVERSAMPLE64</name>
                    <description>64 samples per report</description>
                    <value>0x6</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
              <field>
                <name>CONVERT</name>
                <description>Start conversion</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>YSELECT</name>
            <description>Select Y line</description>
            <addressOffset>0x10</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>YMUX</name>
                <description>Y line selection MUX</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <enumeratedValues>
                  <name>YMUXSelect</name>
                  <enumeratedValue>
                    <name>Y0</name>
                    <description>PTC Y0 Pin</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y1</name>
                    <description>PTC Y1 Pin</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y2</name>
                    <description>PTC Y2 Pin</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y3</name>
                    <description>PTC Y3 Pin</description>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y4</name>
                    <description>PTC Y4 Pin</description>
                    <value>0x10</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y5</name>
                    <description>PTC Y5 Pin</description>
                    <value>0x20</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y6</name>
                    <description>PTC Y6 Pin</description>
                    <value>0x40</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y7</name>
                    <description>PTC Y7 Pin</description>
                    <value>0x80</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y8</name>
                    <description>PTC Y8 Pin</description>
                    <value>0x100</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y9</name>
                    <description>PTC Y9 Pin</description>
                    <value>0x200</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y10</name>
                    <description>PTC Y10 Pin</description>
                    <value>0x400</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y11</name>
                    <description>PTC Y11 Pin</description>
                    <value>0x800</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y12</name>
                    <description>PTC Y12 Pin</description>
                    <value>0x1000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y13</name>
                    <description>PTC Y13 Pin</description>
                    <value>0x2000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y14</name>
                    <description>PTC Y14 Pin</description>
                    <value>0x4000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>Y15</name>
                    <description>PTC Y15 Pin</description>
                    <value>0x8000</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>XSELECT</name>
            <description>Select X line</description>
            <addressOffset>0x12</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>XMUX</name>
                <description>X line selection MUX</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <enumeratedValues>
                  <name>XMUXSelect</name>
                  <enumeratedValue>
                    <name>X0</name>
                    <description>PTC X0 Pin</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X1</name>
                    <description>PTC X1 Pin</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X2</name>
                    <description>PTC X2 Pin</description>
                    <value>0x4</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X3</name>
                    <description>PTC X3 Pin</description>
                    <value>0x8</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X4</name>
                    <description>PTC X4 Pin</description>
                    <value>0x10</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X5</name>
                    <description>PTC X5 Pin</description>
                    <value>0x20</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X6</name>
                    <description>PTC X6 Pin</description>
                    <value>0x40</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X7</name>
                    <description>PTC X7 Pin</description>
                    <value>0x80</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X8</name>
                    <description>PTC X8 Pin</description>
                    <value>0x100</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X9</name>
                    <description>PTC X9 Pin</description>
                    <value>0x200</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X10</name>
                    <description>PTC X10 Pin</description>
                    <value>0x400</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X11</name>
                    <description>PTC X11 Pin</description>
                    <value>0x800</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X12</name>
                    <description>PTC X12 Pin</description>
                    <value>0x1000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X13</name>
                    <description>PTC X13 Pin</description>
                    <value>0x2000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X14</name>
                    <description>PTC X14 Pin</description>
                    <value>0x4000</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>X15</name>
                    <description>PTC X15 Pin</description>
                    <value>0x8000</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>YSELECTEN</name>
            <description>Enable Y lines</description>
            <addressOffset>0x14</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>Y0EN</name>
                <description>PTC Y0 pin enable</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y1EN</name>
                <description>PTC Y1 pin enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y2EN</name>
                <description>PTC Y2 pin enable</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y3EN</name>
                <description>PTC Y3 pin enable</description>
                <bitOffset>3</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y4EN</name>
                <description>PTC Y4 pin enable</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y5EN</name>
                <description>PTC Y5 pin enable</description>
                <bitOffset>5</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y6EN</name>
                <description>PTC Y6 pin enable</description>
                <bitOffset>6</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y7EN</name>
                <description>PTC Y7 pin enable</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y8EN</name>
                <description>PTC Y8 pin enable</description>
                <bitOffset>8</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y9EN</name>
                <description>PTC Y9 pin enable</description>
                <bitOffset>9</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y10EN</name>
                <description>PTC Y10 pin enable</description>
                <bitOffset>10</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y11EN</name>
                <description>PTC Y11 pin enable</description>
                <bitOffset>11</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y12EN</name>
                <description>PTC Y12 pin enable</description>
                <bitOffset>12</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y13EN</name>
                <description>PTC Y13 pin enable</description>
                <bitOffset>13</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y14EN</name>
                <description>PTC Y14 pin enable</description>
                <bitOffset>14</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>Y15EN</name>
                <description>PTC Y15 pin enable</description>
                <bitOffset>15</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>XSELECTEN</name>
            <description>Enable X lines</description>
            <addressOffset>0x16</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>X0EN</name>
                <description>PTC X0 pin enable</description>
                <bitOffset>0</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X1EN</name>
                <description>PTC X1 pin enable</description>
                <bitOffset>1</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X2EN</name>
                <description>PTC X2 pin enable</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X3EN</name>
                <description>PTC X3 pin enable</description>
                <bitOffset>3</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X4EN</name>
                <description>PTC X4 pin enable</description>
                <bitOffset>4</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X5EN</name>
                <description>PTC X5 pin enable</description>
                <bitOffset>5</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X6EN</name>
                <description>PTC X6 pin enable</description>
                <bitOffset>6</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X7EN</name>
                <description>PTC X7 pin enable</description>
                <bitOffset>7</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X8EN</name>
                <description>PTC X8 pin enable</description>
                <bitOffset>8</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X9EN</name>
                <description>PTC X9 pin enable</description>
                <bitOffset>9</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X10EN</name>
                <description>PTC X10 pin enable</description>
                <bitOffset>10</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X11EN</name>
                <description>PTC X11 pin enable</description>
                <bitOffset>11</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X12EN</name>
                <description>PTC X12 pin enable</description>
                <bitOffset>12</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X13EN</name>
                <description>PTC X13 pin enable</description>
                <bitOffset>13</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X14EN</name>
                <description>PTC X14 pin enable</description>
                <bitOffset>14</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>X15EN</name>
                <description>PTC X15 pin enable</description>
                <bitOffset>15</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>COMPCAP</name>
            <description>Compensation capacitor value</description>
            <addressOffset>0x18</addressOffset>
            <size>16</size>
            <fields>
              <field>
                <name>VALUE</name>
                <description>Value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>14</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>INTCAP</name>
            <description>Internal capacitor value</description>
            <addressOffset>0x1A</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>VALUE</name>
                <description>Value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>6</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>SERRES</name>
            <description>Series resistor for PTC measurements</description>
            <addressOffset>0x1B</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>RESISTOR</name>
                <description>Resistor value</description>
                <bitOffset>0</bitOffset>
                <bitWidth>2</bitWidth>
                <enumeratedValues>
                  <name>RESVALUE</name>
                  <enumeratedValue>
                    <name>RES0</name>
                    <description>No series resistor</description>
                    <value>0x0</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES20K</name>
                    <description>20 kiloohm series resistor</description>
                    <value>0x1</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES50K</name>
                    <description>50 kiloohm series resistor</description>
                    <value>0x2</value>
                  </enumeratedValue>
                  <enumeratedValue>
                    <name>RES100K</name>
                    <description>100 kiloohm series resistor</description>
                    <value>0x3</value>
                  </enumeratedValue>
                </enumeratedValues>
              </field>
            </fields>
          </register>
          <register>
            <name>RESULT</name>
            <description>Conversion result</description>
            <addressOffset>0x1C</addressOffset>
            <size>16</size>
            <access>read-only</access>
            <fields>
              <field>
                <name>RESULT</name>
                <description>Result of conversion</description>
                <bitOffset>0</bitOffset>
                <bitWidth>16</bitWidth>
                <access>read-only</access>
              </field>
            </fields>
          </register>
          <register>
            <name>BURSTMODE</name>
            <description>Enable burst or clear to send low power mode</description>
            <addressOffset>0x20</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>CTSLOWPOWEREN</name>
                <description>Enable clear to send low power mode</description>
                <bitOffset>2</bitOffset>
                <bitWidth>1</bitWidth>
              </field>
              <field>
                <name>BURSTMODE</name>
                <description>Burst mode setting</description>
                <bitOffset>4</bitOffset>
                <bitWidth>4</bitWidth>
              </field>
            </fields>
          </register>
          <!-- The rest of these are unused or have unknown uses, but are included for the sake of completeness. -->
          <register>
            <name>WCOMODE</name>
            <description>Set WCO mode</description>
            <addressOffset>0x21</addressOffset>
            <size>8</size>
            <fields>
              <field>
                <name>MODE</name>
                <description>Set WCO mode</description>
                <bitOffset>0</bitOffset>
                <bitWidth>3</bitWidth>
              </field>
            </fields>
          </register>
          <register>
            <name>WCOTHRESHOLDAL</name>
            <description>Set lower WCO threshold for port A</description>
            <addressOffset>0x24</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDAH</name>
            <description>Set upper WCO threshold for port A</description>
            <addressOffset>0x25</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDBL</name>
            <description>Set lower WCO threshold for port B</description>
            <addressOffset>0x26</addressOffset>
            <size>8</size>
          </register>
          <register>
            <name>WCOTHRESHOLDBH</name>
            <description>Set upper WCO threshold for port B</description>
            <addressOffset>0x27</addressOffset>
            <size>8</size>
          </register>
        </registers>
      </peripheral>
    </peripherals>
  </xsl:template>
</xsl:stylesheet>
